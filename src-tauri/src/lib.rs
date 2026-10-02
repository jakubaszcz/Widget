use tauri::Manager;
use tauri_plugin_opener::init;
use windows_sys::w;
use windows_sys::Win32::Foundation::{GetLastError, SetLastError, HWND};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowLongPtrW, SetParent, SetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE, SWP_FRAMECHANGED, SWP_SHOWWINDOW, WS_CHILD, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW, WS_POPUP};

mod tray;
mod widgets;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tray::tray::init(app);
            Ok(())
        })
        .plugin(init())
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
