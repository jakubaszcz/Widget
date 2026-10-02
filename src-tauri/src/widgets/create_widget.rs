use std::sync::atomic::{AtomicUsize, Ordering};
use crate::widgets::attach_widget::attach_widget;

static NEXT_WIDGET_ID: AtomicUsize = AtomicUsize::new(1);
pub fn create_widget(app: &tauri::AppHandle) -> tauri::Result<()> {
    let id  = NEXT_WIDGET_ID.fetch_add(1, Ordering::Relaxed);

    let window = tauri::WebviewWindowBuilder::new(
        app,
        format!("widget_{id}"),
        tauri::WebviewUrl::App("widget.html".into()),
    )
        .title(format!("Widget {id}"))
        .inner_size(200.0, 120.0)
        .decorations(false)
        .shadow(false)
        .transparent(true)
        .resizable(false)
        .skip_taskbar(true)
        .visible(false)
        .build()?;

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