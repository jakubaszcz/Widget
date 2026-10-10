use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use log::error;
use tauri::webview::cookie::time::Error;
use tauri::window::Color;
use crate::global::global::MANIFEST;
use crate::inits::manifest::manifest::{add_widget};
use crate::types::manifest::manifest::{Manifest, ManifestWidget, Vector2};
use crate::widgets::attach_widget::attach_widget;

static NEXT_WIDGET_ID: AtomicUsize = AtomicUsize::new(1);

pub fn load_widget(app: &tauri::AppHandle, id: String, widget_type: String, size: Vector2, position: Vector2) -> tauri::Result<()> {
    let url = match widget_type.to_lowercase().as_str() {
        "note" => "widget.html?widget=note",
        "time" => "widget.html?widget=time",
        "extern" => "widget.html?widget=extern",
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Type de widget inconnu : {}", widget_type.to_lowercase()),
            ).into());
        }
    };
    let window = tauri::WebviewWindowBuilder::new(
        app,
        id.to_string(),
        tauri::WebviewUrl::App(url.into()),
    )
        .title(format!("Widget {id}"))
        .inner_size(size.x as f64, size.y as f64)
        .position(position.x as f64, position.y as f64)
        .decorations(false)
        .shadow(false)
        .transparent(true)
        .resizable(false)
        .maximizable(false)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

    window.set_background_color(Some(
        Color(23, 23, 23, 255),
    ))?;

    let label = window.label().to_string();
    let widget_id = id.clone();

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Moved(position) = event {
            let manifest = &mut MANIFEST.get().unwrap().lock().unwrap();

            if let Some(widget) = manifest.widgets.iter_mut().find(|widget| widget.id == widget_id) {
                widget.position.x = position.x;
                widget.position.y = position.y;
            }
        }

        if let tauri::WindowEvent::Resized(size) = event {
            let manifest = &mut MANIFEST.get().unwrap().lock().unwrap();

            if let Some(widget) = manifest.widgets.iter_mut().find(|widget| widget.id == widget_id) {
                widget.size.x = size.width as i32;
                widget.size.y = size.height as i32;
            }
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
    let id = format!(
        "widget_{}",
        NEXT_WIDGET_ID.fetch_add(1, Ordering::Relaxed)
    );

    load_widget(app, id.to_string(), config.widget.clone(), Vector2 {
        x: config.clone().size.unwrap().x,
        y: config.clone().size.unwrap().y,
    },
    Vector2 { x: 0, y: 0 }
    )?;

    add_widget(ManifestWidget {
        id,
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

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = destination.join(entry.file_name());

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target)?;
        }
    }

    Ok(())
}

pub fn create_extern_widget(
    app: &tauri::AppHandle,
    id: String,
    size: Vector2,
    position: Vector2,
    source: std::path::PathBuf,
) -> Result<bool, String> {
    use crate::global::global::APPDATA;
    use tauri::Manager;

    let id = format!("widget_{id}");
    let existing = MANIFEST.get().unwrap().lock().map_err(|e| e.to_string())?
        .widgets.iter().find(|w| w.id == id).cloned();
    if let Some(widget) = existing {
        if widget.widget_type != "extern" { return Err("Identifiant déjà utilisé".into()); }
        if app.get_webview_window(&id).is_none() {
            load_widget(app, id, widget.widget_type, widget.size, widget.position)
                .map_err(|e| e.to_string())?;
        }
        return Ok(false);
    }
    let folder = APPDATA.get().unwrap().cache.join("templates").join(&id);
    fs::create_dir_all(&folder).map_err(|e| e.to_string())?;

    if !source.join("index.html").is_file() {
        return Err("Le dossier doit contenir index.html".into());
    }

    if folder.canonicalize().map_err(|e| e.to_string())?
        .starts_with(source.canonicalize().map_err(|e| e.to_string())?) {
        return Err("La destination ne doit pas être dans le dossier source".into());
    }

    copy_directory(&source, &folder).map_err(|e| e.to_string())?;

    let html = folder.join("index.html");

    add_widget(ManifestWidget {
        id: id.clone(),
        widget_type: "extern".into(),
        size: size.clone(),
        position: position.clone(),
        data: html,
    })?;

    load_widget(app, id, "extern".into(), size, position)
        .map_err(|e| e.to_string())?;

    Ok(true)
}
