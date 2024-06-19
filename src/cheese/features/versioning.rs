use std::ffi::{c_char, CStr};
use std::ptr;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use crate::cheese::mem::signatures::{scan_sig, SignatureError};
use crate::util::MessageBox;

const VERSION_SIG: &str = "48 8D 2D ?? ?? ?? ?? 48 85 C0 0F 84 70 01 00 00";
const VERSION_OFFSETS: [usize; 2] = [3, 7];

static mut VERSION: *mut c_char = ptr::null_mut();

unsafe fn init_versions() -> Result<(), SignatureError> {
    VERSION = scan_sig(VERSION_SIG, &VERSION_OFFSETS)?;
    Ok(())
}

pub unsafe fn init_versions_or_show_error() -> bool {
    if let Err(e) = init_versions() {
        let text = format!("The essential version signatures weren't found. Please wait for an update.\n\n{:?}", e);
        let title = "Outdated!";
        MessageBox(title, &text, MB_OK);
        return false;
    }
    true
}

#[allow(dead_code)]
pub unsafe fn get_version() -> Option<&'static CStr> {
    if VERSION.is_null() {
        return None;
    }
    Some(CStr::from_ptr(VERSION))
}

#[allow(dead_code)]
pub unsafe fn get_raw_version() -> Option<&'static CStr> {
    if VERSION.is_null() {
        return None;
    }
    Some(CStr::from_ptr(VERSION.add(32)))
}

#[allow(dead_code)]
pub unsafe fn get_online_version() -> Option<&'static CStr> {
    if VERSION.is_null() {
        return None;
    }
    Some(CStr::from_ptr(VERSION.add(64)))
}
