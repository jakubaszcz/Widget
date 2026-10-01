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


#[cfg(target_os = "windows")]
pub fn attach_widget(window: &tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>>{
    unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            SetWindowPos,
            HWND_BOTTOM,
            SWP_NOACTIVATE,
            SWP_NOMOVE,
            SWP_NOSIZE,
        };

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
}