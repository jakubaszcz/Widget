use axum::{
    extract::DefaultBodyLimit,
    http::{header::CONTENT_TYPE, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::net::TcpListener;
use axum::extract::State;
use tauri::AppHandle;
use tower_http::cors::{Any, CorsLayer};
use crate::global::global::MANIFEST;
use crate::server::create_widget::RegisterWidget;
use crate::types::manifest::manifest::Vector2;
use crate::widgets::create_widget::{create_extern_widget, load_widget};

pub const ADDRESS: &str = "127.0.0.1:47832";

pub fn start(app: AppHandle) -> std::io::Result<()> {
    let listener = TcpListener::bind(ADDRESS)?;
    listener.set_nonblocking(true)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![CONTENT_TYPE]);

    let router = Router::new()
        .route("/health", get(health))
        .route("/widgets", post(register_widget))
        .layer(DefaultBodyLimit::max(16 * 1024))
        .with_state(app)
        .layer(cors);

    tauri::async_runtime::spawn(async move {
        let listener = match tokio::net::TcpListener::from_std(listener) {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("Impossible de démarrer l'API Wist : {error}");
                return;
            }
        };

        if let Err(error) = axum::serve(listener, router).await {
            eprintln!("Erreur de l'API Wist : {error}");
        }
    });

    Ok(())
}

async fn health() -> Json<Value> {
    Json(json!({
        "service": "wist",
        "apiVersion": 1
    }))
}


async fn register_widget(
    State(app): State<AppHandle>,
    Json(widget): Json<RegisterWidget>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    println!(
        "Widget reçu : {} ({}) — {} x {}",
        widget.title,
        widget.id,
        widget.width,
        widget.height
    );

    create_extern_widget(
        &app,
        widget.id,
        "extern".into(),
        Vector2 { x: widget.width as i32, y: widget.height as i32 },
        Vector2 { x: 100, y: 100 },
    ).map_err(|error| {
        eprintln!("Impossible de charger le widget externe : {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error.to_string(), "created": false })),
        )
    })?;

    Ok(Json(json!({
        "received": true,
        "created": false
    })))
}
