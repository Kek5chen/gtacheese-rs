use capstone::arch::BuildsCapstone;
use capstone::{arch, Capstone, CsResult};
use std::ffi::c_void;
use std::{mem, ptr};
use thiserror::Error;
use windows::Win32::System::Memory::{VirtualAlloc, VirtualFree, VirtualProtect, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, PAGE_EXECUTE_READ};

#[derive(Error, Debug)]
pub enum HookError {
    #[error("Memory allocation failed")]
    MemAlloc,
    #[error("{0}")]
    Capstone(#[from] capstone::Error),
    #[error("{0}")]
    Windows(#[from] windows::core::Error),
}

pub struct Hooked<T: 'static> {
    pub original: &'static T,
    original_ptr: *const c_void,
    _hook_fn: &'static T,
    bytes_stolen: usize,
}

impl<T> Hooked<T> {
    pub unsafe fn free(self) -> Result<(), HookError> {
        let mut old_prot: PAGE_PROTECTION_FLAGS = Default::default();

        VirtualProtect(
            self.original_ptr,
            self.bytes_stolen,
            PAGE_EXECUTE_READWRITE,
            &mut old_prot,
        )?;
        ptr::copy_nonoverlapping(
            self.original as *const T as *const c_void,
            self.original_ptr as *mut c_void,
            self.bytes_stolen,
        );
        VirtualProtect(
            self.original_ptr,
            self.bytes_stolen,
            old_prot,
            &mut old_prot,
        )?;

        VirtualFree(self.original as *const T as *mut c_void, 0, MEM_RELEASE)?;
        Ok(())
    }
}

unsafe fn set_up_trampoline(
    to_hook: *const c_void,
    hook: *const c_void,
    hook_len: usize,
) -> Result<*mut c_void, HookError> {
    let tramp = VirtualAlloc(
        None,
        hook_len + 5,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    ) as *mut u8;
    if tramp.is_null() {
        return Err(HookError::MemAlloc);
    }

    // stolen bytes
    ptr::copy_nonoverlapping(to_hook, tramp as *mut c_void, hook_len);

    // E9 + relative addr to original function + after stolen bytes
    let jmp_back = tramp.add(hook_len);
    let jmp_to = to_hook.byte_add(hook_len);
    place_jmp(jmp_back as *mut c_void, jmp_to, hook_len);
    
    let mut old_prot: PAGE_PROTECTION_FLAGS = Default::default();
    VirtualProtect(tramp as *const c_void, hook_len, PAGE_EXECUTE_READ, &mut old_prot)?;

    Ok(tramp as *mut c_void)
}

const X86_MAX_FUNCTION_SIZE: usize = 15;

fn determine_hook_length(to_hook: *const c_void) -> CsResult<usize> {
    let code = unsafe { std::slice::from_raw_parts(to_hook as *const u8, X86_MAX_FUNCTION_SIZE) };

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .build()?;

    let insts = cs
        .disasm_all(code, to_hook as u64)
        .expect("Failed to disassemble code");

    let mut length = 0;
    for i in insts.iter() {
        length += i.bytes().len();
        if length >= 5 {
            break;
        }
    }
    Ok(length)
}

unsafe fn place_jmp(to_hook: *mut c_void, jmp_to: *const c_void, hook_len: usize) {
    // place NOPs cuz why not
    ptr::write_bytes(to_hook, 0x90, hook_len);

    let to_hook_b = to_hook as *mut u8;
    *to_hook_b = 0xE9;
    *(to_hook_b.add(1) as *mut u32) = *(to_hook.byte_sub(jmp_to as usize) as *const u32);
}

unsafe fn place_jmp_protected(
    to_hook: *const c_void,
    jmp_to: *mut c_void,
    hook_len: usize,
) -> Result<(), HookError> {
    let mut old_prot: PAGE_PROTECTION_FLAGS = Default::default();

    VirtualProtect(to_hook, hook_len, PAGE_EXECUTE_READWRITE, &mut old_prot)?;

    place_jmp(to_hook as *mut c_void, jmp_to, hook_len);

    VirtualProtect(to_hook, hook_len, old_prot, &mut old_prot)?;

    Ok(())
}

pub unsafe fn hook<O, H>(to_hook: &'static O, hook_fn: &'static H) -> Result<Hooked<O>, HookError> {
    let to_hook_ptr: *const c_void = mem::transmute(to_hook);
    let hook_len = determine_hook_length(to_hook_ptr)?;

    let tramp = set_up_trampoline(to_hook_ptr, mem::transmute(hook_fn), hook_len)?;
    place_jmp_protected(to_hook_ptr, tramp, hook_len)?;

    Ok(Hooked {
        original: &*(tramp as *const O),
        original_ptr: to_hook_ptr,
        _hook_fn: mem::transmute(hook_fn),
        bytes_stolen: hook_len,
    })
}
