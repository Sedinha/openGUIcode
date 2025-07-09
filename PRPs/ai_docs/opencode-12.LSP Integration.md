# Chapter 9: LSP Integration

Welcome back to the OpenCode tutorial! In the last chapter, [Share Feature](08_share_feature_.md), we explored how you can share your interesting AI sessions with others using an external online service and a web view.

Now, let's shift our focus back to how OpenCode interacts more deeply with your local coding environment. AI models are incredibly powerful, but they don't automatically understand the complex rules and structures of every programming language or detect subtle errors in your code like a compiler or a smart code editor does.

Imagine you're asking the AI to help fix a bug in a Go program, but you have a syntax error that prevents the code from even compiling. If the AI just reads the raw text of the file, it might not recognize the error. It needs some "code-aware" information.

This is where the concept of **LSP Integration** comes in.

## What is LSP Integration?

**LSP** stands for **Language Server Protocol**. Think of it as a standard way for code editors (like VS Code, Sublime Text, Neovim) to get "code intelligence" features for different programming languages from separate programs called **Language Servers**.

*   **Language Server:** This is a dedicated program that runs in the background for a specific language (like `gopls` for Go, `tsserver` or `typescript-language-server` for TypeScript/JavaScript, `rust-analyzer` for Rust, etc.). It knows how to analyze code in that language to provide features like:
    *   Finding syntax errors and warnings (Diagnostics)
    *   Providing suggestions for completing code (Autocompletion)
    *   Showing information when you hover over code elements (Hover info)
    *   Finding where a function or variable is used (Find references)
    *   Renaming variables safely (Refactoring)

*   **LSP Client:** This is the part of your code editor (or in our case, OpenCode) that talks *to* the Language Server using the LSP standard. It sends requests ("Hey server, what are the diagnostics for this file?") and receives notifications ("Here are the diagnostics for that file").

**LSP Integration** in OpenCode means that OpenCode can act as an **LSP Client**. It connects to Language Servers that you already have installed on your machine. By talking to these servers, OpenCode can access the same kind of code intelligence that your regular code editor uses, particularly diagnostics (errors and warnings).

This diagnostic information is crucial because it augments the AI's understanding. Instead of just seeing text, OpenCode (and potentially the AI) can know: "Hey, there's a critical syntax error on line 10 of this file," or "This variable is defined but never used." This helps the AI understand the true state of the codebase and generate more accurate responses or suggest relevant actions.

## Why LSP Integration for OpenCode?

*   **Code Awareness:** Gives OpenCode a deeper understanding of the structure and correctness of your code beyond simple text analysis.
*   **Improved AI Accuracy:** Provides the AI with critical information like errors and warnings, helping it understand the context of your coding problems and provide better solutions.
*   **Leverages Existing Tools:** OpenCode uses the same language servers you already use, so it doesn't need to build its own language analysis tools.
*   **Language Support:** By supporting LSP, OpenCode can potentially get code intelligence for *any* language that has an available LSP server.

## Your Use Case: Getting Diagnostics for a File

Let's imagine you're working on a Go project and you make a change to a file, introducing a syntax error. How does OpenCode's LSP integration help detect this?

1.  You (or the AI via a [Tool](04_tools_.md)) interact with a file, perhaps saving a change.
2.  OpenCode's internal logic (potentially triggered by a [Tool](04_tools__.md) like `EditTool` or `WriteTool` or just by opening the file for reading via `ReadTool`) notices that a file has been touched or opened.
3.  OpenCode determines the programming language of the file based on its extension (e.g., `.go` means Go).
4.  OpenCode looks for an available LSP Server configured for that language (e.g., `gopls`).
5.  If a client connection to that LSP Server isn't already active, OpenCode starts the server process and establishes a connection, acting as the LSP Client.
6.  OpenCode, as the LSP Client, sends a standard LSP **notification** to the LSP Server: "Hey `gopls`, I just opened/changed this file: `file:///path/to/your/code.go`". This notification includes the file's content.
7.  The `gopls` server receives the notification, analyzes the file content, and finds the syntax error.
8.  The `gopls` server sends a standard LSP **notification** back to OpenCode (the client): "Here are the diagnostics for `file:///path/to/your/code.go`," including details about the error (message, location, severity).
9.  OpenCode receives this diagnostics notification. It stores the diagnostics internally.
10. This diagnostic information is now available within the [Application Context](06_application_context_.md) or accessible via specific LSP functions. The AI agent can potentially query this information when reasoning about the code, or it can be displayed in the TUI.

## Inside LSP Integration: Key Concepts

OpenCode's LSP integration involves a few key pieces:

*   **LSP Server Definitions:** OpenCode needs to know which LSP servers exist and how to start them for different languages (`packages/opencode/src/lsp/server.ts`).
*   **LSP Client Management:** OpenCode maintains connections to active LSP servers. It acts as the client (`packages/opencode/src/lsp/client.ts`).
*   **Protocol Implementation:** OpenCode uses a library (`vscode-jsonrpc`) to handle the technical details of talking to the servers over their standard input/output streams using the JSON-RPC format defined by LSP.
*   **Diagnostic Storage:** OpenCode stores the diagnostics received from servers internally, mapped to file paths.
*   **Event Notification:** When new diagnostics arrive, OpenCode publishes an event using the [Event Bus](10_event_bus_.md) so other parts of the application can react.

## How LSP Communication Works (Simplified Flow)

Let's visualize the core interaction for getting diagnostics:

```mermaid
sequenceDiagram
    participant Your Code File
    participant Local OpenCode (LSP Client)
    participant LSP Server Process (e.g., gopls)

    Your Code File->>Local OpenCode: File changed/opened (Internal trigger)
    Local OpenCode->>Local OpenCode: Identify Language (.go)
    Local OpenCode->>Local OpenCode: Find Go LSP Server definition
    Local OpenCode->>LSP Server Process: Spawn server process
    Local OpenCode->>Local OpenCode: Establish JSON-RPC connection (stdin/stdout)
    Local OpenCode->>LSP Server Process: LSP Request: "initialize" + capabilities
    LSP Server Process-->>Local OpenCode: LSP Response: "initialize" result
    Local OpenCode->>LSP Server Process: LSP Notification: "initialized"
    Local OpenCode->>LSP Server Process: LSP Notification: "textDocument/didOpen" OR "textDocument/didChange" (with file content)
    LSP Server Process->>LSP Server Process: Analyze file, find diagnostics
    LSP Server Process-->>Local OpenCode: LSP Notification: "textDocument/publishDiagnostics" (with list of errors/warnings)
    Local OpenCode->>Local OpenCode: Store diagnostics internally
    Local OpenCode->>Local OpenCode: Publish "LSP Diagnostics Updated" Event
```

This diagram shows OpenCode starting the server, performing the initial handshake (`initialize`, `initialized`), and then sending a notification (`didOpen` or `didChange`) to trigger the server to analyze the file. The server then sends diagnostics back via the `publishDiagnostics` notification.

## Looking at the Code (Simplified)

Let's peek at the code in `packages/opencode/src/lsp/` to see how this works.

### Defining LSP Servers

OpenCode has a list of known LSP servers it can potentially use in `packages/opencode/src/lsp/server.ts`.

```typescript
// packages/opencode/src/lsp/server.ts (Simplified LSPServer.All)
export namespace LSPServer {
  // ... interfaces and types ...

  export const All: Info[] = [
    {
      id: "typescript", // Unique ID for the server
      extensions: [".ts", ".tsx", ".js", ".jsx", /* ... */], // File extensions it handles
      async spawn(app) { // Function to start the server process
        // Logic to find/install typescript-language-server and tsserver.js
        // ...
        const proc = spawn(
          BunProc.which(), // Command to run (Bun executable)
          ["x", "typescript-language-server", "--stdio"], // Arguments
          { env: { /* ... */ } }, // Environment variables
        )
        return {
          process: proc, // The spawned process handle
          initialization: { /* ... custom init options ... */ }, // Options for LSP handshake
        }
      },
    },
    {
      id: "golang",
      extensions: [".go"],
      async spawn() {
        // Logic to find/install gopls binary
        // ...
        let bin = Bun.which("gopls", { /* ... */ }) // Find gopls executable
        if (!bin) { /* ... install gopls if not found ... */ }
        return {
          process: spawn(bin!), // Spawn the gopls process
        }
      },
    },
    // ... other server definitions (if any) ...
  ]
}
```

`LSPServer.All` is an array defining each supported LSP server. Each definition includes a unique `id`, the file `extensions` it's relevant for, and a `spawn` function. The `spawn` function contains the logic needed to find or install the language server executable and then start it as a separate process using `child_process.spawn`. The `--stdio` argument is standard for LSP servers, telling them to communicate over standard input/output.

### Managing and Connecting Clients

The main `LSP` module (`packages/opencode/src/lsp/index.ts`) manages the active LSP client connections. It uses `App.state` from [Application Context](06_application_context__.md) to keep track of clients globally.

```typescript
// packages/opencode/src/lsp/index.ts (Simplified LSP state)
export namespace LSP {
  // ... log ...

  const state = App.state(
    "lsp", // Key for this state in the Application Context
    async () => {
      log.info("initializing")
      // Map to hold active LSP clients, keyed by server ID
      const clients = new Map<string, LSPClient.Info>()
      const skip = new Set<string>() // Keep track of servers that failed to start
      return {
        clients,
        skip,
      }
    },
    async (state) => {
      // Cleanup function: shut down all active clients when app stops
      for (const client of state.clients.values()) {
        await client.shutdown()
      }
    },
  )

  // Function triggered when a file is touched/opened
  export async function touchFile(input: string, waitForDiagnostics?: boolean) {
    const extension = path.parse(input).ext // Get file extension
    const s = await state() // Get the LSP state (clients map, etc.)
    // Find servers matching the file extension
    const matches = LSPServer.All.filter((x) => x.extensions.includes(extension))

    for (const match of matches) {
      if (s.skip.has(match.id)) continue // Skip if this server failed before
      if (s.clients.has(match.id)) continue // Skip if client is already active

      // If no client exists, spawn the server and create a client
      const handle = await match.spawn(App.info()).catch((e) => {
         log.error(`failed to spawn server ${match.id}`, e)
         return undefined // Handle spawn errors
      })
      if (!handle) {
        s.skip.add(match.id) // Mark server as skipped if spawn failed
        continue
      }

      // Create the LSP client connection
      const client = await LSPClient.create(match.id, handle).catch((e) => {
        log.error(`failed to create client ${match.id}`, e)
        return undefined // Handle client creation errors (e.g., initialize timeout)
      })
      if (!client) {
        s.skip.add(match.id) // Mark server as skipped if client creation failed
        continue
      }
      s.clients.set(match.id, client) // Store the active client

      // Now that the client is created, notify it about the file
      await client.notify.open({ path: input })
      // Optionally wait for diagnostics before returning
      if (waitForDiagnostics) {
         await client.waitForDiagnostics({ path: input })
      }
    }
  }

  // ... other LSP functions (diagnostics, hover, etc.) ...
}
```

The `LSP.state` uses `App.state` to manage a map of active `clients`. The `touchFile` function is a simplified representation of how OpenCode detects a file relevant to LSP. It finds the language server(s) matching the file's extension, checks if a client is already running for that server, and if not, calls the `spawn` function from `LSPServer.All` to start the process. Then, it calls `LSPClient.create` to set up the connection to the new server process and stores the resulting client in the `clients` map. Finally, it sends a `notify.open` to the client, telling the server about the file.

### The LSP Client Implementation

The `LSPClient` (`packages/opencode/src/lsp/client.ts`) is where the actual communication with the LSP server happens.

```typescript
// packages/opencode/src/lsp/client.ts (Simplified LSPClient.create)
export namespace LSPClient {
  // ... log, types, events, errors ...

  export async function create(serverID: string, server: LSPServer.Handle) {
    // Get app info for workspace path
    const app = App.info()

    // Create the JSON-RPC connection using the server process's stdio
    const connection = createMessageConnection(
      new StreamMessageReader(server.process.stdout), // Read from server's standard output
      new StreamMessageWriter(server.process.stdin), // Write to server's standard input
    )

    // Start listening for messages from the server
    connection.listen()

    // --- Handle incoming notifications from the server ---
    const diagnostics = new Map<string, Diagnostic[]>() // Storage for diagnostics
    connection.onNotification("textDocument/publishDiagnostics", (params) => {
      const path = new URL(params.uri).pathname // Extract file path from URI
      log.info("received diagnostics", { path, count: params.diagnostics.length })
      diagnostics.set(path, params.diagnostics) // Store the diagnostics
      Bus.publish(Event.Diagnostics, { path, serverID }) // Publish an event
    })

    // Handle server requests (like workspace config - important for initialization)
    connection.onRequest("workspace/configuration", async () => {
      return [{}] // Return empty config for simplicity
    })

    // --- Perform the LSP initialization handshake ---
    log.info("sending initialize request")
    await connection.sendRequest("initialize", {
      processId: server.process.pid,
      workspaceFolders: [{ name: "workspace", uri: "file://" + app.path.cwd }], // Tell server about the workspace
      initializationOptions: server.initialization, // Pass server-specific options
      capabilities: { /* ... client capabilities we support ... */ }, // Tell server what features we can handle
    })
    await connection.sendNotification("initialized", {}) // Send initialized notification
    log.info("initialization complete")

    // --- Provide functions for OpenCode to interact with the client ---
    const files: { [path: string]: number } = {} // To track file versions

    const result = {
      serverID: serverID,
      connection: connection, // Expose the connection (used internally by LSP module)
      notify: {
        async open(input: { path: string }) {
          // Logic to read file content and send didOpen or didChange notification
          // ... reads file using Bun.file ...
          await connection.sendNotification("textDocument/didOpen", { /* ... */ }) // OR didChange
          files[input.path] = (files[input.path] || 0) + 1 // Increment version
          log.info("sent didOpen/didChange", input)
        },
        // ... other notifications if needed ...
      },
      get diagnostics() {
        return diagnostics // Expose the stored diagnostics
      },
      async waitForDiagnostics(input: { path: string }) {
        // Helper to wait for a specific diagnostics event for a file
        // ... uses Bus.subscribe and a timeout ...
      },
      async shutdown() {
        // Logic to shut down the connection and kill the process
        // ... connection.end(), connection.dispose(), server.process.kill() ...
        log.info("client shutdown complete")
      },
    }

    return result // Return the client object ready for use
  }
}
```

The `LSPClient.create` function is where the magic happens. It takes the server's process handle and:

1.  Sets up a `createMessageConnection` using `StreamMessageReader` and `StreamMessageWriter` attached to the server's standard output and input. This is how they communicate.
2.  Starts listening for messages using `connection.listen()`.
3.  Sets up a handler (`connection.onNotification`) specifically for `"textDocument/publishDiagnostics"`. When the server sends diagnostics, this handler stores them in the `diagnostics` map (keyed by file path) and publishes an `LSPClient.Event.Diagnostics` event using the [Event Bus](10_event_bus_.md).
4.  Performs the standard LSP `initialize` and `initialized` handshake requests to tell the server about the workspace and client capabilities.
5.  Returns an object (`result`) with methods like `notify.open` (which sends the file content to the server via `didOpen` or `didChange` notifications) and a getter for the stored `diagnostics`.

### Accessing Diagnostics

Other parts of OpenCode (or potentially the AI via a dedicated query mechanism) can access the stored diagnostics using functions in the main `LSP` module:

```typescript
// packages/opencode/src/lsp/index.ts (Simplified LSP.diagnostics)
export namespace LSP {
  // ... state and touchFile ...

  // Function to get all diagnostics from all active clients
  export async function diagnostics() {
    const results: Record<string, LSPClient.Diagnostic[]> = {}
    // Get diagnostics from each active client
    for (const client of (await state()).clients.values()) {
      for (const [path, diagnostics] of client.diagnostics.entries()) {
        const arr = results[path] || []
        arr.push(...diagnostics)
        results[path] = arr
      }
    }
    return results // Return combined diagnostics by file path
  }

  export namespace Diagnostic {
    // Helper function to format a diagnostic message nicely
    export function pretty(diagnostic: LSPClient.Diagnostic) {
      const severityMap = { 1: "ERROR", 2: "WARN", 3: "INFO", 4: "HINT" }
      const severity = severityMap[diagnostic.severity || 1]
      const line = diagnostic.range.start.line + 1 // LSP is 0-indexed lines
      const col = diagnostic.range.start.character + 1 // LSP is 0-indexed columns
      return `${severity} [${line}:${col}] ${diagnostic.message}` // e.g., "ERROR [10:5] Missing semicolon"
    }
  }

  // ... hover and run functions ...
}
```

The `LSP.diagnostics()` function iterates through all active LSP clients stored in the `state` and collects the diagnostics map from each client. It combines them into a single result object, making it easy for other parts of OpenCode to get a comprehensive list of current problems across relevant files. The `Diagnostic.pretty` helper shows how you might format this raw diagnostic data into a human-readable string.

While the AI doesn't directly call these `LSP` functions, the information they provide (especially the diagnostics) can be passed to the AI model as part of the conversation context or environment description, giving the AI deeper insight into the code it's helping with.

## Conclusion

LSP Integration is OpenCode's way of gaining "code awareness" by connecting to standard Language Servers you already use. By acting as an LSP client, OpenCode can receive valuable information like errors and warnings (diagnostics) for your code files. This information enhances the AI's understanding of the codebase, enabling it to provide more relevant and accurate assistance. OpenCode manages the process of finding, spawning, connecting to, and communicating with these servers using the LSP protocol, making diagnostic information available internally and via events.

Many parts of OpenCode coordinate and communicate using events. Understanding how these events flow is key to grasping how the different pieces of the application work together.

[Next Chapter: Event Bus](10_event_bus_.md)

---

<sub><sup>Generated by [AI Codebase Knowledge Builder](https://github.com/The-Pocket/Tutorial-Codebase-Knowledge).</sup></sub> <sub><sup>**References**: [[1]](https://github.com/sst/opencode/blob/c5eefd17528fd03a5c2553c8bf9d5c931597e09c/packages/opencode/src/lsp/client.ts), [[2]](https://github.com/sst/opencode/blob/c5eefd17528fd03a5c2553c8bf9d5c931597e09c/packages/opencode/src/lsp/index.ts), [[3]](https://github.com/sst/opencode/blob/c5eefd17528fd03a5c2553c8bf9d5c931597e09c/packages/opencode/src/lsp/language.ts), [[4]](https://github.com/sst/opencode/blob/c5eefd17528fd03a5c2553c8bf9d5c931597e09c/packages/opencode/src/lsp/server.ts)</sup></sub>
````