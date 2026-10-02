use crate::widgets::attach_widget::attach_widget;

pub fn create_widget_creator(app: &tauri::AppHandle) -> tauri::Result<()> {
    let _ = tauri::WebviewWindowBuilder::new(
        app,
        "widget_creator".to_string(),
        tauri::WebviewUrl::App("creator.html".into()),
    )
        .title("Widget Creator".to_string())
        .decorations(false)
        .resizable(false)
        .shadow(false)
        .inner_size(500.0, 400.0)
        .build()?;

    Ok(())
}