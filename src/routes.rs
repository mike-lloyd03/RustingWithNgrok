use askama_axum::IntoResponse;
use axum::{
    extract::{
        ws::{self, Message},
        State, WebSocketUpgrade,
    },
    http::HeaderMap,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

use crate::AppState;
use rustingwithngrok::{ChatMsg, IndexTemplate};

/// Renders and returns the `index.html` template
pub async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let chat_messages = state.get_history();
    IndexTemplate { chat_messages }
}

/// Handles websocket connections. Usernames are derived from the `ngrok-auth-user-name`
/// header.
pub async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let username = match headers.get("ngrok-auth-user-name") {
        Some(header) => header.to_str().unwrap_or("None").to_string(),
        None => "None".to_string(),
    };
    ws.on_upgrade(|socket| chat_service(socket, state, username))
}

/// Handles both sending and receiving websocket messages
async fn chat_service(socket: ws::WebSocket, state: Arc<AppState>, username: String) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    log::info!("{} connected", username);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let u = username.clone();

    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let msg = ChatMsg {
                username: u.clone(),
                text,
            };

            _ = tx.send(msg.to_string());

            state.push_history(msg);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => receive_task.abort(),
        _ = (&mut receive_task) => send_task.abort(),
    };
    log::info!("{} disconnected", username);
}
