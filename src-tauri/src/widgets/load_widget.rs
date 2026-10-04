use crate::global::global::MANIFEST;
use crate::widgets::create_widget::load_widget;

pub fn load_widgets(app: &tauri::AppHandle) -> tauri::Result<()> {
    let widgets = MANIFEST.get().unwrap().lock().unwrap().widgets.clone();

    for widget in widgets {
        load_widget(app, widget.id, widget.widget_type, widget.size, widget.position)?;
    }

    Ok(())
}
