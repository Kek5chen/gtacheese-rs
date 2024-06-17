use std::ffi::{ CStr, CString};
use std::time::Duration;

const VERSION_SIG: &str = "48 8D 2D ?? ?? ?? ?? 48 85 C0 0F 84 70 01 00 00";
const VERSION_OFFSETS: [usize; 2] = [3, 7];
const RAW_VERSION_OFFSETS: [usize; 2] = [3, 7 + 32];
const ONLINE_VERSION_OFFSETS: [usize; 2] = [3, 7 + 64];

unsafe fn get_version_inner(offsets: &[usize]) -> &'static CStr {
    let ptr = scan_sig(VERSION_SIG, Some(offsets));
    if ptr.is_none() {
        log::error!("Game Version not found from signature. Dying inside :<");
        return CStr::from_ptr(CString::new("").unwrap().into_raw());
    }
    CStr::from_ptr(ptr.unwrap())
}

pub unsafe fn get_version() -> &'static CStr {
    get_version_inner(&VERSION_OFFSETS)
}

pub unsafe fn get_raw_version() -> &'static CStr {
    get_version_inner(&RAW_VERSION_OFFSETS)
}

pub unsafe fn get_online_version() -> &'static CStr {
    get_version_inner(&ONLINE_VERSION_OFFSETS)
}
