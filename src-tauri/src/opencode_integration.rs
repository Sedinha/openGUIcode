use anyhow::{Context, Result};
use futures::stream::StreamExt;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

/// Global state to track the OpenCode server process
pub struct OpenCodeState {
    pub server_process: Arc<Mutex<Option<Child>>>,
    pub server_info: Arc<Mutex<Option<OpenCodeServerInfo>>>,
    pub http_client: Client,
}

impl Default for OpenCodeState {
    fn default() -> Self {
        Self {
            server_process: Arc::new(Mutex::new(None)),
            server_info: Arc::new(Mutex::new(None)),
            http_client: Client::new(),
        }
    }
}

/// Information about the running OpenCode server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeServerInfo {
    pub port: u16,
    pub hostname: String,
    pub pid: Option<u32>,
    pub status: ServerStatus,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    Starting,
    Running,
    Stopped,
    Error(String),
}

/// OpenCode session information matching the TypeScript interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeSession {
    pub id: String,
    #[serde(rename = "parentID")]
    pub parent_id: Option<String>,
    pub share: Option<ShareInfo>,
    pub title: String,
    pub version: String,
    pub time: TimeInfo,
    pub revert: Option<RevertInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareInfo {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInfo {
    pub created: u64,
    pub updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevertInfo {
    #[serde(rename = "messageID")]
    pub message_id: String,
    pub part: u32,
    pub snapshot: Option<String>,
}

/// OpenCode message information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeMessage {
    pub id: String,
    pub role: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub parts: Vec<MessagePart>,
    pub time: TimeInfo,
    #[serde(rename = "modelID")]
    pub model_id: Option<String>,
    #[serde(rename = "providerID")]
    pub provider_id: Option<String>,
    pub cost: Option<f64>,
    pub tokens: Option<TokenInfo>,
    pub system: Option<Vec<String>>,
    pub path: Option<PathInfo>,
    pub summary: Option<bool>,
    pub error: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub input: u32,
    pub output: u32,
    pub reasoning: u32,
    pub cache: CacheInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub read: u32,
    pub write: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathInfo {
    pub cwd: String,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool")]
    Tool {
        tool: String,
        id: String,
        state: ToolState,
    },
    #[serde(rename = "file")]
    File {
        url: String,
        mime: String,
        filename: String,
    },
    #[serde(rename = "step-start")]
    StepStart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ToolState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running {
        input: serde_json::Value,
        time: ToolTimeInfo,
        title: Option<String>,
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "completed")]
    Completed {
        input: serde_json::Value,
        output: String,
        time: ToolTimeInfo,
        title: Option<String>,
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "error")]
    Error {
        input: Option<serde_json::Value>,
        error: String,
        time: ToolTimeInfo,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTimeInfo {
    pub start: u64,
    pub end: Option<u64>,
}

/// Chat request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
    pub parts: Vec<UserMessagePart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserMessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "file")]
    File {
        url: String,
        mime: String,
        filename: String,
    },
}

/// SSE event from OpenCode server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenCodeEvent {
    #[serde(rename = "message.updated")]
    MessageUpdated {
        info: OpenCodeMessage,
    },
    #[serde(rename = "message.part.updated")]
    MessagePartUpdated {
        part: MessagePart,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
    },
    #[serde(rename = "message.removed")]
    MessageRemoved {
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
    },
    #[serde(rename = "session.updated")]
    SessionUpdated {
        info: OpenCodeSession,
    },
    #[serde(rename = "session.deleted")]
    SessionDeleted {
        info: OpenCodeSession,
    },
    #[serde(rename = "session.idle")]
    SessionIdle {
        #[serde(rename = "sessionID")]
        session_id: String,
    },
    #[serde(rename = "session.error")]
    SessionError {
        #[serde(rename = "sessionID")]
        session_id: Option<String>,
        error: serde_json::Value,
    },
}

impl OpenCodeState {
    /// Start the OpenCode server process
    pub async fn start_server(&self, app_handle: &AppHandle) -> Result<OpenCodeServerInfo> {
        let mut server_process = self.server_process.lock().await;
        
        // Check if server is already running
        if let Some(mut child) = server_process.take() {
            if let Ok(Some(_)) = child.try_wait() {
                // Process has exited, continue with starting a new one
            } else {
                // Process is still running, return existing info
                let server_info = self.server_info.lock().await;
                if let Some(info) = server_info.as_ref() {
                    return Ok(info.clone());
                }
            }
        }

        log::info!("Starting OpenCode server...");

        // Find bun binary
        let bun_path = which::which("bun")
            .or_else(|_| which::which("node"))
            .context("Could not find bun or node binary")?;

        // Get the OpenCode path (relative to the Claudia binary)
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .context("Could not get app data directory")?;
        let opencode_path = app_dir
            .parent()
            .context("Could not get parent directory")?
            .join("opencode")
            .join("packages")
            .join("opencode")
            .join("src")
            .join("index.ts");

        if !opencode_path.exists() {
            return Err(anyhow::anyhow!(
                "OpenCode server not found at: {}",
                opencode_path.display()
            ));
        }

        // Create command
        let mut cmd = Command::new(&bun_path);
        cmd.arg("run")
            .arg(&opencode_path)
            .arg("serve")
            .arg("--port")
            .arg("0") // Let the OS choose an available port
            .arg("--hostname")
            .arg("127.0.0.1")
            .current_dir(
                opencode_path
                    .parent()
                    .context("Could not get OpenCode directory")?,
            )
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Spawn the process
        let mut child = cmd.spawn().context("Failed to spawn OpenCode server")?;
        let pid = child.id();

        // Get stdout to read the port number
        let stdout = child.stdout.take().context("Failed to get stdout")?;
        
        // Store the child process
        *server_process = Some(child);

        // Parse stdout to get the server port
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut line = String::new();
        let mut port = 0u16;
        
        // Try to read the port from stdout for up to 10 seconds
        for _ in 0..100 {
            line.clear();
            match tokio::time::timeout(
                Duration::from_millis(100),
                tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut line),
            )
            .await
            {
                Ok(Ok(0)) => break, // EOF
                Ok(Ok(_)) => {
                    log::debug!("OpenCode stdout: {}", line.trim());
                    // Look for port in the output
                    if let Some(port_str) = extract_port_from_line(&line) {
                        port = port_str;
                        break;
                    }
                }
                Ok(Err(e)) => {
                    log::error!("Error reading OpenCode stdout: {}", e);
                    break;
                }
                Err(_) => {
                    // Timeout, continue waiting
                    continue;
                }
            }
        }

        if port == 0 {
            // Fallback: try common ports
            port = 3001; // Default port for development
        }

        let hostname = "127.0.0.1".to_string();
        let base_url = format!("http://{}:{}", hostname, port);

        let server_info = OpenCodeServerInfo {
            port,
            hostname,
            pid,
            status: ServerStatus::Starting,
            base_url,
        };

        // Wait for server to be ready
        let ready_info = self.wait_for_server_ready(server_info).await?;

        // Store server info
        let mut info_guard = self.server_info.lock().await;
        *info_guard = Some(ready_info.clone());

        log::info!(
            "OpenCode server started successfully on {}:{}",
            ready_info.hostname,
            ready_info.port
        );

        app_handle
            .emit("opencode-server-started", &ready_info)
            .context("Failed to emit server started event")?;

        Ok(ready_info)
    }

    /// Wait for the server to be ready by testing the /app endpoint
    async fn wait_for_server_ready(&self, mut info: OpenCodeServerInfo) -> Result<OpenCodeServerInfo> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 30; // 30 seconds max
        
        while attempts < MAX_ATTEMPTS {
            attempts += 1;
            
            match self.test_server_connection(&info.base_url).await {
                Ok(_) => {
                    info.status = ServerStatus::Running;
                    return Ok(info);
                }
                Err(e) => {
                    log::debug!("Server not ready yet (attempt {}): {}", attempts, e);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }

        info.status = ServerStatus::Error("Server failed to start within timeout".to_string());
        Err(anyhow::anyhow!("OpenCode server failed to start within timeout"))
    }

    /// Test connection to the server
    async fn test_server_connection(&self, base_url: &str) -> Result<()> {
        let response = self
            .http_client
            .get(&format!("{}/app", base_url))
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .context("Failed to connect to server")?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Server returned error status: {}",
                response.status()
            ))
        }
    }

    /// Stop the OpenCode server
    pub async fn stop_server(&self) -> Result<()> {
        let mut server_process = self.server_process.lock().await;
        
        if let Some(mut child) = server_process.take() {
            log::info!("Stopping OpenCode server...");
            
            // Try graceful shutdown first
            if let Err(e) = child.kill().await {
                log::error!("Failed to kill OpenCode server: {}", e);
            }
            
            // Wait for process to exit
            match child.wait().await {
                Ok(status) => {
                    log::info!("OpenCode server stopped with status: {}", status);
                }
                Err(e) => {
                    log::error!("Error waiting for OpenCode server to stop: {}", e);
                }
            }
        }

        // Clear server info
        let mut info_guard = self.server_info.lock().await;
        if let Some(ref mut info) = info_guard.as_mut() {
            info.status = ServerStatus::Stopped;
        } else {
            *info_guard = None;
        }

        Ok(())
    }

    /// Get current server information
    pub async fn get_server_info(&self) -> Option<OpenCodeServerInfo> {
        self.server_info.lock().await.clone()
    }

    /// Send a chat message to OpenCode
    pub async fn send_chat_message(
        &self,
        session_id: &str,
        request: ChatRequest,
    ) -> Result<OpenCodeMessage> {
        let server_info = self
            .server_info
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenCode server not running"))?
            .clone();

        let url = format!("{}/session/{}/message", server_info.base_url, session_id);
        
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send chat message")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Chat request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let message: OpenCodeMessage = response
            .json()
            .await
            .context("Failed to parse chat response")?;

        Ok(message)
    }

    /// Create a new session in OpenCode
    pub async fn create_session(&self) -> Result<OpenCodeSession> {
        let server_info = self
            .server_info
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenCode server not running"))?
            .clone();

        let url = format!("{}/session", server_info.base_url);
        
        let response = self
            .http_client
            .post(&url)
            .send()
            .await
            .context("Failed to create session")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Session creation failed with status {}: {}",
                status,
                error_text
            ));
        }

        let session: OpenCodeSession = response
            .json()
            .await
            .context("Failed to parse session response")?;

        Ok(session)
    }

    /// List all sessions from OpenCode
    pub async fn list_sessions(&self) -> Result<Vec<OpenCodeSession>> {
        let server_info = self
            .server_info
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenCode server not running"))?
            .clone();

        let url = format!("{}/session", server_info.base_url);
        
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to list sessions")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Session list failed with status {}: {}",
                status,
                error_text
            ));
        }

        let sessions: Vec<OpenCodeSession> = response
            .json()
            .await
            .context("Failed to parse sessions response")?;

        Ok(sessions)
    }

    /// Get messages for a session
    pub async fn get_session_messages(&self, session_id: &str) -> Result<Vec<OpenCodeMessage>> {
        let server_info = self
            .server_info
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenCode server not running"))?
            .clone();

        let url = format!("{}/session/{}/message", server_info.base_url, session_id);
        
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .context("Failed to get session messages")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Get messages failed with status {}: {}",
                status,
                error_text
            ));
        }

        let messages: Vec<OpenCodeMessage> = response
            .json()
            .await
            .context("Failed to parse messages response")?;

        Ok(messages)
    }

    /// Connect to the OpenCode event stream
    pub async fn connect_event_stream(&self, app_handle: AppHandle) -> Result<()> {
        let server_info = self
            .server_info
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenCode server not running"))?
            .clone();

        let url = format!("{}/event", server_info.base_url);
        
        log::info!("Connecting to OpenCode event stream at {}", url);
        
        let response = self
            .http_client
            .get(&url)
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .send()
            .await
            .context("Failed to connect to event stream")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Event stream connection failed with status: {}",
                response.status()
            ));
        }

        // Process the event stream
        tokio::spawn(async move {
            if let Err(e) = Self::process_event_stream(response, app_handle).await {
                log::error!("Event stream processing error: {}", e);
            }
        });

        Ok(())
    }

    /// Process the SSE event stream
    async fn process_event_stream(response: Response, app_handle: AppHandle) -> Result<()> {
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.push_str(&chunk_str);

            // Process complete SSE events
            while let Some(event_end) = buffer.find("\n\n") {
                let event_str = buffer[..event_end].to_string();
                buffer = buffer[event_end + 2..].to_string();

                if let Err(e) = Self::handle_sse_event(&event_str, &app_handle).await {
                    log::error!("Failed to handle SSE event: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Handle a single SSE event
    async fn handle_sse_event(event_str: &str, app_handle: &AppHandle) -> Result<()> {
        // Parse SSE format: "data: {...}"
        for line in event_str.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim().is_empty() || data == "{}" {
                    continue;
                }

                match serde_json::from_str::<OpenCodeEvent>(data) {
                    Ok(event) => {
                        Self::emit_opencode_event(event, app_handle).await?;
                    }
                    Err(e) => {
                        log::debug!("Failed to parse OpenCode event: {} - Data: {}", e, data);
                        // Emit raw event for debugging
                        app_handle
                            .emit("opencode-raw-event", data)
                            .context("Failed to emit raw event")?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Emit OpenCode events as Tauri events
    async fn emit_opencode_event(event: OpenCodeEvent, app_handle: &AppHandle) -> Result<()> {
        match event {
            OpenCodeEvent::MessageUpdated { info } => {
                app_handle
                    .emit("opencode-message-updated", &info)
                    .context("Failed to emit message updated event")?;
                
                // Also emit with session isolation
                app_handle
                    .emit(&format!("opencode-message-updated:{}", info.session_id), &info)
                    .context("Failed to emit session-specific message updated event")?;
            }
            OpenCodeEvent::MessagePartUpdated { part, session_id, message_id } => {
                let payload = serde_json::json!({
                    "part": part,
                    "sessionId": session_id,
                    "messageId": message_id
                });
                
                app_handle
                    .emit("opencode-message-part-updated", &payload)
                    .context("Failed to emit message part updated event")?;
                
                // Also emit with session isolation
                app_handle
                    .emit(&format!("opencode-message-part-updated:{}", session_id), &payload)
                    .context("Failed to emit session-specific message part updated event")?;
            }
            OpenCodeEvent::SessionUpdated { info } => {
                app_handle
                    .emit("opencode-session-updated", &info)
                    .context("Failed to emit session updated event")?;
            }
            OpenCodeEvent::SessionDeleted { info } => {
                app_handle
                    .emit("opencode-session-deleted", &info)
                    .context("Failed to emit session deleted event")?;
            }
            OpenCodeEvent::SessionIdle { session_id } => {
                app_handle
                    .emit("opencode-session-idle", &session_id)
                    .context("Failed to emit session idle event")?;
                
                // Also emit with session isolation
                app_handle
                    .emit(&format!("opencode-session-idle:{}", session_id), &session_id)
                    .context("Failed to emit session-specific session idle event")?;
            }
            OpenCodeEvent::SessionError { session_id, error } => {
                let payload = serde_json::json!({
                    "sessionId": session_id,
                    "error": error
                });
                
                app_handle
                    .emit("opencode-session-error", &payload)
                    .context("Failed to emit session error event")?;
                
                // Also emit with session isolation if session_id is present
                if let Some(sid) = &session_id {
                    app_handle
                        .emit(&format!("opencode-session-error:{}", sid), &payload)
                        .context("Failed to emit session-specific session error event")?;
                }
            }
            _ => {
                // Handle other event types or emit as generic event
                app_handle
                    .emit("opencode-event", &event)
                    .context("Failed to emit generic OpenCode event")?;
            }
        }

        Ok(())
    }
}

/// Helper function to extract port number from server output
fn extract_port_from_line(line: &str) -> Option<u16> {
    // Look for patterns like "Server listening on :3001" or "localhost:3001"
    let patterns = [
        r":(\d+)",
        r"port (\d+)",
        r"listening.*?(\d+)",
        r"localhost:(\d+)",
        r"127\.0\.0\.1:(\d+)",
    ];

    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(line) {
                if let Some(port_match) = captures.get(1) {
                    if let Ok(port) = port_match.as_str().parse::<u16>() {
                        return Some(port);
                    }
                }
            }
        }
    }

    None
}