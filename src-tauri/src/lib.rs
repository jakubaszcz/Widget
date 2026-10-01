use tauri::Manager;
use windows_sys::w;
use windows_sys::Win32::Foundation::{GetLastError, SetLastError, HWND};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowLongPtrW, SetParent, SetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE, SWP_FRAMECHANGED, SWP_SHOWWINDOW, WS_CHILD, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW, WS_POPUP};

#[cfg(target_os = "windows")]
unsafe fn set_parent_window(hwnd: HWND) {
    let desktop_host = unsafe {
        FindWindowW(
            w!("Progman"),
            std::ptr::null(),
        )
    };

    if desktop_host.is_null() {
        return
    }

    SetParent(hwnd, desktop_host);
}

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
            #[cfg(target_os = "windows")]
            unsafe {
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

                let hwnd: HWND = window.hwnd()?.0 as _;

                SetLastError(0);
                let styles = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                let error = GetLastError();

                {
                    if styles == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }
                }

                let new_styles = (styles | WS_EX_TOOLWINDOW as isize)
                    & !(WS_EX_APPWINDOW  as isize);

                SetLastError(0);
                let previous = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_styles);
                let error = GetLastError();

                {
                    if previous == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }
                }

                SetLastError(0);
                let styles = GetWindowLongPtrW(hwnd, GWL_STYLE);
                let error = GetLastError();

                {
                    if styles == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }
                }

                let new_child = (styles | WS_CHILD as isize) & !(WS_POPUP as isize);
                let previous = SetWindowLongPtrW(hwnd, GWL_STYLE, new_child);
                let error = GetLastError();

                {
                    if previous == 0 && error != 0 {
                        return Err(
                            std::io::Error::from_raw_os_error(error as i32).into()
                        );
                    }
                }

                let success = SetWindowPos(
                    hwnd,
                    HWND_BOTTOM,
                    0, 0,
                    0, 0,
                    SWP_NOMOVE
                        | SWP_NOSIZE
                        | SWP_NOACTIVATE
                        | SWP_FRAMECHANGED
                        | SWP_SHOWWINDOW,
                );

                {
                    #[cfg(target_os = "windows")]
                    set_parent_window(hwnd);
                }

                {
                    if success == 0 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                }

                Ok(())
            }
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
