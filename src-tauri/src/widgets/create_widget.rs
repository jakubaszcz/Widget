use std::sync::atomic::{AtomicUsize, Ordering};
use log::error;
use tauri::webview::cookie::time::Error;
use crate::inits::manifest::manifest::{add_widget};
use crate::types::manifest::manifest::{Manifest, ManifestWidget, Vector2};
use crate::widgets::attach_widget::attach_widget;

static NEXT_WIDGET_ID: AtomicUsize = AtomicUsize::new(1);

pub fn load_widget(app: &tauri::AppHandle, id: String, widget_type: String, size: Vector2, _position: Vector2) -> tauri::Result<()> {
    let url = match widget_type.to_lowercase().as_str() {
        "note" => "widget.html?widget=note",
        "time" => "widget.html?widget=time",
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Type de widget inconnu : {}", widget_type.to_lowercase()),
            ).into());
        }
    };
    let window = tauri::WebviewWindowBuilder::new(
        app,
        format!("widget_{id}"),
        tauri::WebviewUrl::App(url.into()),
    )
        .title(format!("Widget {id}"))
        .inner_size(size.x as f64, size.y as f64)
        .decorations(false)
        .shadow(false)
        .transparent(true)
        .resizable(false)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    let label = window.label().to_string();

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Moved(position) = event {
            println!(
                "Widget {} déplacé : x={}, y={}",
                label,
                position.x,
                position.y
            );
        }
    });

    let widget = window.clone();

    if let Err(error) = window.run_on_main_thread(move || {
        if let Err(error) = attach_widget(&widget) {
            eprintln!("Error attaching widget {id}: {}", error);
            let _ = widget.destroy();
        }
    }) {
        let _ = window.destroy();
        return Err(error);
    }
    Ok(())
}
pub fn create_widget(
    app: &tauri::AppHandle,
    config: crate::types::widgets::widgets::CreateWidget) -> tauri::Result<()> {
    let id  = NEXT_WIDGET_ID.fetch_add(1, Ordering::Relaxed);

    load_widget(app, id.to_string(), config.widget.clone(), Vector2 {
        x: config.clone().size.unwrap().x,
        y: config.clone().size.unwrap().y,
    },
    Vector2 { x: 0, y: 0 }
    )?;

    add_widget(ManifestWidget {
        id: id.to_string(),
        widget_type: config.widget.to_string(),
        size: Vector2 {
            x: config.clone().size.unwrap().x,
            y: config.clone().size.unwrap().y,
        },
        position: Vector2 {
            x: config.clone().size.unwrap().x,
            y: config.clone().size.unwrap().y,
        },
        data: Default::default(),
    }).unwrap();
    Ok(())
}
