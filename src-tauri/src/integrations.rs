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
use crate::widgets::create_widget::create_extern_widget;

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
        .route("/templates/{id}/{*file}", get(template_asset))
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

    if widget.id.is_empty() || widget.id.len() > 80
        || !widget.id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        || !(120..=4096).contains(&widget.width) || !(80..=4096).contains(&widget.height) {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error":"Identifiant ou dimensions invalides (120–4096 × 80–4096)"}))));
    }
    let created = tauri::async_runtime::spawn_blocking(move || create_extern_widget(
        &app, widget.id,
        Vector2 { x: widget.width as i32, y: widget.height as i32 },
        Vector2 { x: 100, y: 100 }, widget.path,
    )).await.map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":error.to_string()}))))?
    .map_err(|error| {
        eprintln!("Impossible de charger le widget externe : {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error.to_string(), "created": false })),
        )
    })?;

    Ok(Json(json!({
        "received": true,
        "created": created
    })))
}

async fn template_asset(
    axum::extract::Path((id, file)): axum::extract::Path<(String, String)>,
) -> Result<impl axum::response::IntoResponse, StatusCode> {
    use crate::global::global::APPDATA;
    use std::path::{Path, Component};
    if file.contains(['\\', ':']) || !Path::new(&file).components().all(|c| matches!(c, Component::Normal(_))) {
        return Err(StatusCode::FORBIDDEN);
    }
    let entry = MANIFEST.get().unwrap().lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .widgets.iter().find(|w| w.id == id && w.widget_type == "extern")
        .map(|w| w.data.clone()).ok_or(StatusCode::NOT_FOUND)?;
    let cache = APPDATA.get().unwrap().cache.join("templates").canonicalize().map_err(|_| StatusCode::NOT_FOUND)?;
    let root = entry.parent().ok_or(StatusCode::NOT_FOUND)?.canonicalize().map_err(|_| StatusCode::NOT_FOUND)?;
    if !root.starts_with(&cache) { return Err(StatusCode::FORBIDDEN); }
    let path = root.join(file).canonicalize().map_err(|_| StatusCode::NOT_FOUND)?;
    if !path.starts_with(&root) || !path.is_file() { return Err(StatusCode::FORBIDDEN); }
    let content_type = match path.extension().and_then(|v| v.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "svg" => "image/svg+xml", "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg", "webp" => "image/webp",
        "gif" => "image/gif", "ico" => "image/x-icon",
        "woff" => "font/woff", "woff2" => "font/woff2", "ttf" => "font/ttf",
        _ => return Err(StatusCode::FORBIDDEN),
    };
    let bytes = std::fs::read(path).map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(([
        ("content-type", content_type),
        ("x-content-type-options", "nosniff"),
        ("cache-control", "no-store"),
        ("content-security-policy", "sandbox allow-scripts; default-src 'none'; script-src http://127.0.0.1:47832/templates/; style-src 'unsafe-inline' http://127.0.0.1:47832/templates/; img-src data: http://127.0.0.1:47832/templates/; font-src data: http://127.0.0.1:47832/templates/; connect-src http://127.0.0.1:47832/templates/; base-uri 'none'; form-action 'none'"),
        ("access-control-allow-origin", "*"),
    ], bytes))
}
