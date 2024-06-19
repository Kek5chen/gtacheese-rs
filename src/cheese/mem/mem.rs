use std::ffi::c_void;
use windows::Win32::System::Memory::IsBadReadPtr;

pub unsafe fn is_addr_valid(addr: usize) -> bool {
    if addr == 0 {
        return false;
    }
    !IsBadReadPtr(Some(addr as *const c_void), 1).as_bool()
}