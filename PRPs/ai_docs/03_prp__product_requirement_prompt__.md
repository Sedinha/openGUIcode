# Chapter 3: PRP (Product Requirement Prompt)

Welcome back! In the last chapter, [Chapter 2: PRP Execution (Running a PRP)](02_prp_execution__running_a_prp__.md), you learned how to tell Claude Code to take a detailed plan – something we call a PRP – and work on it automatically using a special command like `/project:PRPs:prp-base-execute`. You saw that the AI goes through steps like understanding the task, planning, coding, and validating its work.

But what exactly *is* this "PRP" document that guides the AI? That's what we'll explore in this chapter.

## What Problem Does the PRP Solve?

Imagine you want an AI coding assistant to build a new feature. If you just say, "Build user authentication," what happens?

*   The AI might ask lots of questions: "What kind of auth? Password? OAuth? JWT? Where should the code go? What libraries should I use? How should I handle errors? How will I know it's working correctly?"
*   Or worse, it might just guess and build something that doesn't fit your project at all.

Traditional project documents, like Product Requirements Documents (PRDs), tell you *what* to build and *why*, but deliberately leave the *how* to the engineers. This works for humans, but an AI agent needs more explicit guidance to integrate seamlessly into an existing codebase and meet specific technical standards.

The **PRP (Product Requirement Prompt)** is designed to solve this. It's not just *what* needs doing, but also provides the *how*, the *context*, and the *checks* needed for an AI to do the job right the first time.

## What is a PRP?

Think of a PRP as a **super-detailed work order** specifically tailored for an AI developer.

It combines the clarity of a traditional product requirement with all the technical context and specific instructions an AI agent needs to execute the task autonomously.

As the project's README puts it:

> A PRP is PRD + curated codebase intelligence + agent/runbook—the minimum viable packet an AI needs to plausibly ship production-ready code on the first pass.

Let's break down what this means and how it differs from a traditional PRD:

| Feature             | Traditional PRD                               | PRP (Product Requirement Prompt)                       |
| :------------------ | :-------------------------------------------- | :----------------------------------------------------- |
| **Focus**           | What to build & Why                           | What, Why, **How**, **Context**, **Validation**      |
| **Target Audience** | Product Managers, Engineers (Human)           | **AI Agents**, Engineers (Human)                       |
| **"How" Details**   | Generally avoided                             | **Explicitly defined** (blueprint, pseudocode)         |
| **Code Context**    | Implicit (engineers know the codebase)        | **Explicitly included** (file references, docs, gotchas) |
| **Success Check**   | Manual review, separate testing process       | **Executable validation steps included in document**   |
| **Goal**            | Define the product feature                    | **Enable AI to implement feature autonomously**      |

A PRP takes the *goals* and *requirements* from a PRD and adds crucial layers of technical guidance and context, making it executable by an AI agent.

## The Key Components of a PRP

PRPs in this project follow a specific structure, usually based on a template like `PRPs/templates/prp_base.md`. This structure is designed to provide the AI with information in a logical flow. While the exact sections can vary, a base PRP typically includes:

1.  **Goal, Why, What:**
    *   `## Goal`: The main objective of the task. What should the final outcome look like?
    *   `## Why`: Explains the purpose. Why is this feature needed? What problem does it solve? This helps the AI understand the importance and context.
    *   `## What`: Describes the specific requirements and desired user behavior. What exactly should the AI build? What are the success criteria?
    *   *Analogy:* These sections are like the brief summary and objectives a project manager gives you – the "big picture" requirements.

2.  **All Needed Context:**
    *   This is where the "curated codebase intelligence" comes in. This section provides the AI with the specific information it needs about the existing project, libraries, and best practices.
    *   Examples of context you might include:
        *   Links to relevant documentation URLs (`url:`)
        *   Paths to existing files in your codebase that show patterns to follow or code to reference (`file:`)
        *   References to documentation files curated specifically for the AI (`docfile:`, often in the `PRPs/ai_docs/` directory). See [Codebase Context (AI Documentation)](07_codebase_context__ai_documentation__.md).
        *   Important "gotchas" or quirks of your specific codebase or libraries (`Known Gotchas:`) - things that might trip up an AI (or a human!) if they don't know about them.
    *   *Analogy:* This is like giving the AI a stack of relevant design documents, snippets of existing code to copy the style from, and warnings about tricky parts of the system.

    Here's a snippet from the template showing how context is listed:

    ```yaml
    ## All Needed Context

    ### Documentation & References
    ```yaml
    # MUST READ - Include these in your context window
    - url: https://fastapi.tiangolo.com/tutorial/security/oauth2-jwt/
      why: Essential guide for JWT auth pattern

    - file: src/api/base.py
      why: Shows our standard API router setup

    - docfile: PRPs/ai_docs/db_guidelines.md
      why: Our specific ORM usage patterns and anti-patterns

    ```
    ```
    (Remember, these are just examples of *how* you list context; Claude Code's internal tools handle reading the file contents or summaries when processing the PRP during execution, as discussed in Chapter 1 with the `@` and `!` symbols, although those symbols are used in the command files *triggering* the execution, not necessarily directly in the PRP `yaml` itself. The *command* file's instructions tell Claude Code to *load* the context listed in the PRP.)*

3.  **Implementation Blueprint:**
    *   This section tells the AI *how* to build the feature, providing a step-by-step plan and sometimes pseudocode.
    *   It breaks the task down into logical steps (`List of tasks:`), specifying actions like `CREATE file.py`, `MODIFY existing.py`, `INJECT after line...`, etc.
    *   For complex steps, you can include pseudocode (`Per task pseudocode:`), highlighting critical details, patterns to follow, or anti-patterns to avoid.
    *   *Analogy:* This is like giving the AI a detailed technical specification or a sequence diagram, outlining the exact classes to create, functions to modify, and the basic logic flow, including specific coding style notes.

    Snippet from the template:

    ```yaml
    ## Implementation Blueprint

    ### List of tasks to be completed
    ```yaml
    Task 1:
    CREATE src/auth/jwt.py:
      - Implement token encoding/decoding
      - Follow pattern from file: src/auth/basic.py

    Task 2:
    MODIFY src/api/auth_routes.py:
      - Add login endpoint
      - Use functions from src/auth/jwt.py
    ```

    ### Per task pseudocode as needed
    ```python
    # Task 2 Pseudocode
    async def login(credentials):
        user = authenticate_user(credentials) # Use existing function
        if not user:
            raise HTTPException(status_code=401)

        # CRITICAL: Use RS256 algorithm (see Known Gotchas)
        access_token = create_jwt(user.id, algorithm="RS256")
        # PATTERN: Store refresh token in httpOnly cookie (see Context)
        response.set_cookie("refresh_token", create_refresh_token(user.id), httponly=True)

        return {"access_token": access_token}
    ```
    Notice how the pseudocode isn't full, executable code, but focuses on the *critical* details and references the context or patterns mentioned elsewhere in the PRP.

4.  **Validation Loop:**
    *   This section is absolutely vital for autonomous AI development. It provides executable commands that the AI can run to check its own work.
    *   The AI runs these commands after it thinks it has completed a task or set of tasks. If the commands fail (e.g., a test breaks, linting fails), the AI analyzes the output and attempts to fix its code. This loop repeats until the validation steps pass.
    *   The template suggests levels of validation (Syntax/Style, Unit Tests, Integration Tests), but you can include any command that gives clear, deterministic feedback (e.g., `ruff check`, `mypy`, `pytest`, `curl` requests).
    *   *Analogy:* This is like giving the AI a checklist of automated tests or quality checks it *must* pass before considering the task done. If it fails a check, it knows it needs to go back and fix something. This is so important it gets its own chapter: [Validation Loops](04_validation_loops_.md).

    Snippet from the template:

    ```bash
    ## Validation Loop

    ### Level 1: Syntax & Style
    ```bash
    ruff check --fix src/auth/jwt.py  # Check specific file, auto-fix
    mypy src/auth/                   # Check type hints in directory
    ```

    ### Level 2: Unit Tests
    ```bash
    uv run pytest tests/auth/test_jwt.py -v
    ```

    ### Level 3: Integration Test
    ```bash
    # Requires service to be running
    curl -X POST http://localhost:8000/login \
      -H "Content-Type: application/json" \
      -d '{"username": "testuser", "password": "password123"}' | jq
    ```
    ```

5.  **Final Validation Checklist:**
    *   A simple checklist of items (often repeats from Validation Loop steps or adds manual checks) for the AI to confirm before declaring completion.
    *   *Analogy:* The final sign-off checklist.

6.  **Anti-Patterns to Avoid:**
    *   Explicitly listing common mistakes or patterns specific to your team/codebase that the AI should avoid.
    *   *Analogy:* The "lessons learned" or "common pitfalls" document.

By structuring the PRP this way, you give the AI a complete picture: *what* needs to be done, *why* it's important, *where* to find relevant information, *how* to approach the implementation, and *how* to verify its own work.

## Solving Our Use Case with a PRP

Let's revisit our goal: getting Claude Code to build a feature (like the user profile endpoint from Chapter 2).

To achieve this, you would **create a PRP file** (e.g., `PRPs/implement-user-profile.md`) following the structure above. You would fill in:

*   The `Goal`, `Why`, and `What` sections describing the endpoint.
*   The `All Needed Context` section, pointing to your database schema file, existing API code examples, user model definitions, and maybe any library documentation for your web framework or ORM.
*   The `Implementation Blueprint`, outlining steps like "create `src/api/users.py`", "define the GET route", "call the database function", "handle errors".
*   The `Validation Loop`, including commands to run linters (`ruff`, `mypy`), unit tests (`pytest`), and maybe a `curl` command to test the live endpoint once the service is running.
*   Any relevant `Known Gotchas` or `Anti-Patterns`.

Once this `implement-user-profile.md` file is ready, you trigger the execution using the command we learned in Chapter 2:

```bash
/project:PRPs:prp-base-execute PRPs/implement-user-profile.md
```

As you saw in the previous chapter, this command tells Claude Code to load its execution runbook (`.claude/commands/PRPs/prp-base-execute.md`) and use *your specific PRP file* (`PRPs/implement-user-profile.md`) as the task definition. The AI then follows the steps outlined in the command, reading and utilizing the information provided in each section of your PRP file to plan, code, and validate the feature implementation.

## Under the Hood: How the AI Uses the PRP

When the AI executes a PRP via the `/project:PRPs:prp-base-execute` command, it doesn't just read the file top-to-bottom and start coding. It uses the structured sections to inform its process:

1.  **Initial Read & Understanding:** The AI reads the entire PRP, paying close attention to the `Goal`, `Why`, and `What` to grasp the overall objective and requirements.
2.  **Context Loading:** It processes the `All Needed Context` section. It uses internal tools to read the content of referenced files (`@`), summaries of documentation (`url:`, `docfile:`), and understands the `Known Gotchas`. This context is loaded into its working memory or made available for reference.
3.  **Planning (`ULTRATHINK`):** Using the `Goal`, `What`, and the loaded `Context`, the AI analyzes the `Implementation Blueprint`. It uses this to create a detailed step-by-step plan for coding the feature, potentially using internal task management tools (like the `TodoWrite` tool mentioned in the project's files).
4.  **Execution & Iteration:** The AI starts implementing the plan, writing or modifying code according to the `Implementation Blueprint` and referencing the loaded `Context` and `Anti-Patterns` as it codes.
5.  **Validation & Self-Correction:** Periodically, or after completing a task from the blueprint, the AI moves to the `Validation Loop` section. It executes the specified commands.
    *   If commands pass, it moves on to the next task or declares completion.
    *   If commands fail, it reads the error output, analyzes the failure using its understanding of the `Implementation Blueprint` and `Context`, identifies the likely cause, fixes the code, and runs the validation again. This loop continues until validation passes.
6.  **Final Check:** Once validation passes, it reviews the `Final validation Checklist` to ensure all criteria are met.

Here's a simplified flow:

```mermaid
sequenceDiagram
    participant User
    participant ClaudeCodeTool
    participant FileSystem
    participant ClaudeAPI

    User->>ClaudeCodeTool: Run /execute-base-prp my-prp.md
    ClaudeCodeTool->>FileSystem: Read my-prp.md
    FileSystem-->>ClaudeCodeTool: Return PRP content
    ClaudeCodeTool->>ClaudeAPI: Send PRP content + Execution command instructions
    ClaudeAPI->>ClaudeAPI: Understand Goal, Why, What
    ClaudeAPI->>ClaudeAPI: Load Context (ref files, docs)
    ClaudeAPI->>ClaudeAPI: Plan based on Blueprint (ULTRATHINK)
    loop Execution and Validation
        ClaudeAPI->>ClaudeAPI: Implement code
        ClaudeAPI->>ClaudeCodeTool: Execute Validation commands from PRP
        ClaudeCodeTool->>ClaudeAPI: Return Validation Results (pass/fail, errors)
        alt Validation Failed
            ClaudeAPI->>ClaudeAPI: Debug and Fix code
        else Validation Passed
            break
        end
    end
    ClaudeAPI->>ClaudeAPI: Check Final Checklist
    ClaudeAPI-->>ClaudeCodeTool: Report Completion
    ClaudeCodeTool->>User: Display Completion Status
```

The structure of the PRP file is crucial because it provides this clear roadmap and necessary resources for the AI agent to navigate the development process with minimal external guidance.

## Conclusion

In this chapter, you learned that a **PRP (Product Requirement Prompt)** is a comprehensive work order for an AI agent, going far beyond a traditional PRD. It includes the goals and requirements but also crucial technical context, implementation guidance, and executable validation steps.

You saw how the different sections of a PRP (Goal, Why, What, Context, Implementation Blueprint, Validation Loop) provide the AI agent with all the information it needs to understand the task, plan its approach, implement the code, and critically, self-correct by running tests and checks.

Understanding the structure and purpose of the PRP document is key to effectively using this framework. You now know that running a command like `/project:PRPs:prp-base-execute` is essentially handing this detailed work order to the AI and telling it to follow the instructions within, using the built-in execution process.

In the next chapter, we'll take a much closer look at one of the most powerful sections of the PRP: the **[Validation Loops](04_validation_loops_.md)**, and understand why they are so critical for achieving reliable, autonomous code generation.

[Validation Loops](04_validation_loops_.md)

---

<sub><sup>Generated by [AI Codebase Knowledge Builder](https://github.com/The-Pocket/Tutorial-Codebase-Knowledge).</sup></sub> <sub><sup>**References**: [[1]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/CLAUDE.md), [[2]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/PRPs/README.md), [[3]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/PRPs/templates/prp_base.md), [[4]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/README.md)</sup></sub>
````