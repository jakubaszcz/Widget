use std::fs;
use std::io::ErrorKind;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::init;
use crate::global::global::{APPDATA, MANIFEST};
use crate::manifest::auto_save;
use crate::types::widgets::data::note_widget::NoteData;

mod tray;
mod widgets;
mod types;

mod inits;
mod global;
mod manifest;
/*#[cfg(target_os = "windows")]
unsafe fn inspect_desktop_windows() -> std::io::Result<()> {

    use std::io::Write;

    use windows_sys::Win32::{
        Foundation::{HWND, LPARAM},
        UI::WindowsAndMessaging::{
            EnumChildWindows, EnumWindows,
            GetClassNameW, GetParent, IsWindowVisible,
        },
    };

    fn class_name(hwnd: HWND) -> String {
        let mut buffer = [0u16; 256];

        let length = unsafe {
            GetClassNameW(
                hwnd,
                buffer.as_mut_ptr(),
                buffer.len() as i32
            )
        };

        String::from_utf16_lossy(&buffer[..length as usize])
    }
    unsafe extern "system" fn inspect_child(hwnd: HWND, _: LPARAM) -> i32 {
        let class = class_name(hwnd);
        let parent = unsafe { GetParent(hwnd) };
        let visible = unsafe { IsWindowVisible(parent) };


        let _ = writeln!(
            std::io::stdout(),
            "  descendant {:?} | {} | parent {:?} | visible {}",
            hwnd, class, parent, visible,
        );

        1
    }

    unsafe extern "system" fn inspect_root(hwnd: HWND, _: LPARAM) -> i32 {
        let class = class_name(hwnd);

        if !class.is_empty() {
            let visible = unsafe { IsWindowVisible(hwnd) != 0};

            let _ = writeln!(
                std::io::stdout(),
                "\nCANDIDATE {:?} | {} | visible {}",
                hwnd, class, visible,
            );

            unsafe {
                EnumChildWindows(hwnd, Some(inspect_child), 0);
            }
        }

        1
    }

    if unsafe { EnumWindows(Some(inspect_root), 0) } == 0 {
        return Err(std::io::Error::last_os_error());
    }


    Ok(())
}*/

#[tauri::command]
fn save_note_widget(id: String, data: NoteData) -> Result<(), String> {
    println!("Widget: {id}, data: {data:?}");
    inits::manifest::widgets::widget_manifest::init(id, &data)?;
    Ok(())
}

#[tauri::command]
async fn create_widget(
    app: AppHandle,
    config: types::widgets::widgets::CreateWidget,
) -> Result<(), String> {
    widgets::create_widget::create_widget(&app, config)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn load_widget_data(id: String) -> Result<Option<serde_json::Value>, String> {
    let cache = &APPDATA.get().unwrap().cache;
    let path = cache.join(format!("{id}.json"));

    match fs::read(path) {
        Ok(contents) => {
            let data = serde_json::from_slice::<serde_json::Value>(&contents)
                .map_err(|err| err.to_string())?;

            Ok(Some(data))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn delete_widget(id: String) -> Result<(), String> {
    let mut manifest = MANIFEST.get().unwrap().lock()
        .map_err(|err| err.to_string())?;

    if let Some(widget) = manifest.widgets.iter()
        .find(|widget| widget.id == id)
    {
        if !widget.data.as_os_str().is_empty() {
            match fs::remove_file(&widget.data) {
                Ok(()) => {}
                Err(err) if err.kind() == ErrorKind::NotFound => {}
                Err(err) => return Err(err.to_string()),
            }
        }
    }

    manifest.widgets.retain(|widget| widget.id != id);

    let path = APPDATA.get().unwrap().data.join("manifest.json");
    inits::manifest::manifest::write_manifest(&path, &manifest)?;

    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    inits::inits::inits();

    tauri::Builder::default()
        .setup(|app| {
            tray::tray::init(app);
            widgets::load_widget::load_widgets(app.handle())?;
            auto_save::auto_save();
            Ok(())
        })
        .plugin(init())
        .invoke_handler(tauri::generate_handler![create_widget, save_note_widget, load_widget_data, delete_widget])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            if let tauri::RunEvent::ExitRequested { code, api, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
