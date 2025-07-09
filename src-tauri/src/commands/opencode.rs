use anyhow::Result;
use tauri::{AppHandle, State};

use crate::opencode_integration::{
    ChatRequest, OpenCodeMessage, OpenCodeServerInfo, OpenCodeSession, OpenCodeState,
    UserMessagePart,
};

/// Start the OpenCode server
#[tauri::command]
pub async fn start_opencode_server(
    app: AppHandle,
    state: State<'_, OpenCodeState>,
) -> Result<OpenCodeServerInfo, String> {
    log::info!("Starting OpenCode server...");
    
    state
        .start_server(&app)
        .await
        .map_err(|e| format!("Failed to start OpenCode server: {}", e))
}

/// Stop the OpenCode server
#[tauri::command]
pub async fn stop_opencode_server(
    state: State<'_, OpenCodeState>,
) -> Result<(), String> {
    log::info!("Stopping OpenCode server...");
    
    state
        .stop_server()
        .await
        .map_err(|e| format!("Failed to stop OpenCode server: {}", e))
}

/// Get current OpenCode server status
#[tauri::command]
pub async fn get_opencode_server_status(
    state: State<'_, OpenCodeState>,
) -> Result<Option<OpenCodeServerInfo>, String> {
    Ok(state.get_server_info().await)
}

/// Create a new OpenCode session
#[tauri::command]
pub async fn create_opencode_session(
    state: State<'_, OpenCodeState>,
) -> Result<OpenCodeSession, String> {
    state
        .create_session()
        .await
        .map_err(|e| format!("Failed to create OpenCode session: {}", e))
}

/// List all OpenCode sessions
#[tauri::command]
pub async fn list_opencode_sessions(
    state: State<'_, OpenCodeState>,
) -> Result<Vec<OpenCodeSession>, String> {
    state
        .list_sessions()
        .await
        .map_err(|e| format!("Failed to list OpenCode sessions: {}", e))
}

/// Get messages for an OpenCode session
#[tauri::command]
pub async fn get_opencode_session_messages(
    session_id: String,
    state: State<'_, OpenCodeState>,
) -> Result<Vec<OpenCodeMessage>, String> {
    state
        .get_session_messages(&session_id)
        .await
        .map_err(|e| format!("Failed to get session messages: {}", e))
}

/// Send a chat message to OpenCode
#[tauri::command]
pub async fn send_opencode_chat_message(
    session_id: String,
    message: String,
    provider_id: String,
    model_id: String,
    state: State<'_, OpenCodeState>,
) -> Result<OpenCodeMessage, String> {
    let request = ChatRequest {
        provider_id,
        model_id,
        parts: vec![UserMessagePart::Text { text: message }],
    };

    state
        .send_chat_message(&session_id, request)
        .await
        .map_err(|e| format!("Failed to send chat message: {}", e))
}

/// Connect to OpenCode event stream
#[tauri::command]
pub async fn connect_opencode_event_stream(
    app: AppHandle,
    state: State<'_, OpenCodeState>,
) -> Result<(), String> {
    state
        .connect_event_stream(app)
        .await
        .map_err(|e| format!("Failed to connect to event stream: {}", e))
}

/// Execute OpenCode chat (similar to execute_claude_code but using OpenCode)
#[tauri::command]
pub async fn execute_opencode_chat(
    app: AppHandle,
    project_path: String,
    prompt: String,
    model: String,
    provider: Option<String>,
    state: State<'_, OpenCodeState>,
) -> Result<OpenCodeSession, String> {
    log::info!(
        "Starting OpenCode chat in: {} with model: {} (provider: {:?})",
        project_path,
        model,
        provider
    );

    // Ensure server is running
    let _server_info = match state.get_server_info().await {
        Some(info) => info,
        None => {
            // Start server if not running
            state
                .start_server(&app)
                .await
                .map_err(|e| format!("Failed to start OpenCode server: {}", e))?
        }
    };

    // Create a new session
    let session = state
        .create_session()
        .await
        .map_err(|e| format!("Failed to create session: {}", e))?;

    // Connect to event stream if not already connected
    if let Err(e) = state.connect_event_stream(app.clone()).await {
        log::warn!("Failed to connect event stream: {}", e);
        // Don't fail the entire operation for event stream connection
    }

    // Send the initial message
    let provider_id = provider.unwrap_or_else(|| "anthropic".to_string());
    let request = ChatRequest {
        provider_id,
        model_id: model,
        parts: vec![UserMessagePart::Text { text: prompt }],
    };

    // Send the chat message
    let _message = state
        .send_chat_message(&session.id, request)
        .await
        .map_err(|e| format!("Failed to send initial message: {}", e))?;

    log::info!("OpenCode chat session started successfully: {}", session.id);

    Ok(session)
}

/// Continue an OpenCode conversation (similar to continue_claude_code)
#[tauri::command]
pub async fn continue_opencode_chat(
    session_id: String,
    prompt: String,
    model: String,
    provider: Option<String>,
    state: State<'_, OpenCodeState>,
) -> Result<OpenCodeMessage, String> {
    log::info!(
        "Continuing OpenCode chat session: {} with model: {} (provider: {:?})",
        session_id,
        model,
        provider
    );

    let provider_id = provider.unwrap_or_else(|| "anthropic".to_string());
    let request = ChatRequest {
        provider_id,
        model_id: model,
        parts: vec![UserMessagePart::Text { text: prompt }],
    };

    let message = state
        .send_chat_message(&session_id, request)
        .await
        .map_err(|e| format!("Failed to continue chat: {}", e))?;

    log::info!("OpenCode chat continued successfully");

    Ok(message)
}

/// Abort an OpenCode session
#[tauri::command]
pub async fn abort_opencode_session(
    session_id: String,
    state: State<'_, OpenCodeState>,
) -> Result<bool, String> {
    log::info!("Aborting OpenCode session: {}", session_id);

    let server_info = state
        .get_server_info()
        .await
        .ok_or_else(|| "OpenCode server not running".to_string())?;

    let url = format!("{}/session/{}/abort", server_info.base_url, session_id);
    
    let response = state
        .http_client
        .post(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to abort session: {}", e))?;

    if response.status().is_success() {
        let result: bool = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse abort response: {}", e))?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(format!(
            "Abort request failed with status {}: {}",
            status,
            error_text
        ))
    }
}