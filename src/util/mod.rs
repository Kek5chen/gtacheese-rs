use std::ffi::CString;
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_RESULT, MESSAGEBOX_STYLE, MessageBoxA};

pub unsafe fn MessageBox(title: &str, text: &str, style: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
    let c_title = CString::new(title);
    let c_text = CString::new(text);
    
    if let (Ok(title), Ok(text)) = (c_title, c_text) {
        let lptext = PCSTR(title.as_ptr() as *const u8);
        let lpcaption = PCSTR(text.as_ptr() as *const u8);

        MessageBoxA(None, lptext, lpcaption, style)
    } else {
        MESSAGEBOX_RESULT(0)
    }
}