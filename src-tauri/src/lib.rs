use tauri::Manager;
use windows_sys::Win32::Foundation::{GetLastError, SetLastError};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {

                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    SetWindowPos,
                    HWND_BOTTOM,
                    SWP_NOACTIVATE,
                    SWP_NOMOVE,
                    SWP_NOSIZE,
                };

                let window = app
                    .get_webview_window("main")
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Main windows not found."
                        )
                    })?;

                let hwnd = window.hwnd()?.0 as _;

                unsafe {
                    SetLastError(1);
                    let styles = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    let error = GetLastError();

                    if styles == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }

                    let new_styles = (styles | WS_EX_TOOLWINDOW as isize)
                    & !(WS_EX_APPWINDOW  as isize);

                    SetLastError(0);
                    let previous = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_styles);
                    let error = GetLastError();

                    if previous == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }

                    let success = unsafe {
                        SetWindowPos(
                            hwnd,
                            HWND_BOTTOM,
                            0, 0,
                            0, 0,
                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                        )
                    };

                    if success == 0 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                }

                Ok(())

            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
