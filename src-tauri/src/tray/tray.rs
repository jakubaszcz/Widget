use tauri::App;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use crate::tray::create_widget_creator::create_widget_creator;
use crate::widgets::create_widget::create_widget;

pub fn init(app: &mut App) {
    let create = MenuItem::with_id(app, "create", "create widget", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&create, &quit]).unwrap();

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Widget")
        .icon(
            app.default_window_icon()
                .expect("Icône de l'application absente")
                .clone()
        )
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "create" => {
                #[cfg(target_os = "windows")]
                {
                    let app = app.clone();

                    std::thread::spawn(move || {
                        if let Err(error) = create_widget_creator(&app) {
                            eprintln!("Création du widget impossible : {error}");
                        }
                    });
                }
            }
            _ => {}
        })
        .build(app).unwrap();
}