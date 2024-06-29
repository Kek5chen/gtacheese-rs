use std::ffi::{c_char, CStr, CString};
use std::ptr;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use crate::cheese::main::PROC;
use crate::cheese::mem::signatures::{SignatureError};
use crate::util::MessageBox;

const VERSION_SIG: &str = "48 8D 2D ?? ?? ?? ?? 48 85 C0 0F 84 70 01 00 00";
const VERSION_OFFSETS: [usize; 2] = [3, 7];
const VERSION_SIZE: usize = 32;

static mut VERSION: *mut c_char = ptr::null_mut();

unsafe fn init_versions() -> Result<(), SignatureError> {
    VERSION = PROC.scan_sig(VERSION_SIG, &VERSION_OFFSETS)?;
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
pub unsafe fn get_version() -> Option<String> {
    if VERSION.is_null() {
        return None;
    }

    let version = PROC.read_raw(VERSION as usize, VERSION_SIZE)?;
    Some(CStr::from_bytes_until_nul(&version).unwrap().to_string_lossy().to_string())
}

#[allow(dead_code)]
pub unsafe fn get_raw_version() -> Option<String> {
    if VERSION.is_null() {
        return None;
    }

    let version = PROC.read_raw(VERSION as usize + VERSION_SIZE, VERSION_SIZE)?;
    Some(CStr::from_bytes_until_nul(&version).ok()?.to_string_lossy().to_string())
}

#[allow(dead_code)]
pub unsafe fn get_online_version() -> Option<String> {
    if VERSION.is_null() {
        return None;
    }

    let version = PROC.read_raw(VERSION as usize + VERSION_SIZE * 2, VERSION_SIZE)?;
    Some(CStr::from_bytes_until_nul(&version).ok()?.to_string_lossy().to_string())
}
