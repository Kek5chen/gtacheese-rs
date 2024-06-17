use std::ffi::CStr;
use crate::cheese::mem::signatures::{scan_sig, SignatureError};

const VERSION_SIG: &str = "48 8D 2D ?? ?? ?? ?? 48 85 C0 0F 84 70 01 00 00";
const VERSION_OFFSETS: [usize; 2] = [3, 7];
const RAW_VERSION_OFFSETS: [usize; 2] = [3, 7 + 32];
const ONLINE_VERSION_OFFSETS: [usize; 2] = [3, 7 + 64];

unsafe fn get_version_inner(offsets: &[usize]) -> Result<&'static CStr, SignatureError> {
    let ptr = scan_sig(VERSION_SIG, offsets)?;
    Ok(CStr::from_ptr(ptr))
}

#[allow(dead_code)]
pub unsafe fn get_version() -> Result<&'static CStr, SignatureError> {
    get_version_inner(&VERSION_OFFSETS)
}

#[allow(dead_code)]
pub unsafe fn get_raw_version() -> Result<&'static CStr, SignatureError> {
    get_version_inner(&RAW_VERSION_OFFSETS)
}

#[allow(dead_code)]
pub unsafe fn get_online_version() -> Result<&'static CStr, SignatureError> {
    get_version_inner(&ONLINE_VERSION_OFFSETS)
}
