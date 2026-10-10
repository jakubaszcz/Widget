use std::{collections::HashMap, sync::{Mutex, OnceLock}};
use axum::{extract::{Query, ws::{WebSocketUpgrade, Message}}, http::StatusCode, response::Response};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::global::global::MANIFEST;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetAction {
    pub r#type: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

static CHANNELS: OnceLock<Mutex<HashMap<String, broadcast::Sender<WidgetAction>>>> = OnceLock::new();

fn channel(id: &str) -> Result<broadcast::Sender<WidgetAction>, String> {
    let mut channels = CHANNELS.get_or_init(Default::default).lock().map_err(|e| e.to_string())?;
    Ok(channels.entry(id.to_owned()).or_insert_with(|| broadcast::channel(32).0).clone())
}

#[derive(Deserialize)]
pub struct Subscription {
    #[serde(rename = "widgetId")]
    widget_id: String,
}

pub async fn events(Query(query): Query<Subscription>, ws: WebSocketUpgrade) -> Result<Response, StatusCode> {
    let id = query.widget_id;
    if id.is_empty() || id.len() > 80 || !id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_') {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut receiver = channel(&id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.subscribe();
    Ok(ws.on_upgrade(move |mut socket| async move {
        loop {
            tokio::select! {
                incoming = socket.recv() => {
                    match incoming {
                        Some(Ok(Message::Close(_))) | Some(Err(_)) | None => break,
                        Some(Ok(Message::Ping(data))) => {
                            if socket.send(Message::Pong(data)).await.is_err() { break; }
                        }
                        _ => {}
                    }
                }
                action = receiver.recv() => {
                    match action {
                        Ok(action) => {
                            let message = serde_json::json!({"type":"widget-action", "widgetId":id, "action":action});
                            if socket.send(Message::Text(message.to_string().into())).await.is_err() { break; }
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => break,
                    }
                }
            }
        }
    }))
}

#[tauri::command]
pub fn send_widget_action(window: tauri::Window, action: WidgetAction) -> Result<(), String> {
    if action.r#type.is_empty() || action.r#type.len() > 100 || serde_json::to_vec(&action).map_err(|e| e.to_string())?.len() > 16 * 1024 {
        return Err("Invalid widget action".into());
    }
    let manifest = MANIFEST.get().ok_or("Manifest unavailable")?.lock().map_err(|e| e.to_string())?;
    if !manifest.widgets.iter().any(|w| w.id == window.label() && w.widget_type == "extern") {
        return Err("Unknown external widget".into());
    }
    let id = window.label().strip_prefix("widget_").ok_or("Invalid widget label")?;
    channel(id)?.send(action).map_err(|_| "Application disconnected".to_string())?;
    Ok(())
}
