use std::ffi::c_void;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

pub unsafe fn get_base_addr() -> usize {
    GetModuleHandleW(None).unwrap().0 as usize // executable base address should always be found
}
