use capstone::arch::x86::X86OperandType;
use capstone::arch::{BuildsCapstone, DetailsArchInsn};
use capstone::{arch, Capstone, Instructions};
use std::ffi::c_void;
use std::ptr;
use thiserror::Error;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, VirtualProtect, MEM_COMMIT, MEM_RELEASE, 
    PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};

#[derive(Error, Debug)]
pub enum HookError {
    #[error("Memory allocation failed")]
    MemAlloc,
    #[error("{0}")]
    Capstone(#[from] capstone::Error),
    #[error("{0}")]
    Windows(#[from] windows::core::Error),
    #[error("Can't hook into section with relative operand inside of instruction")]
    Relative,
}

pub struct Hooked<T: 'static> {
    pub(crate) original: *const T,
    original_ptr: *const c_void,
    _hook_fn: *const T,
    bytes_stolen: usize,
}

const X86_MAX_FUNCTION_SIZE: usize = 15;
const JMP_SIZE: usize = 12;

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
            self.original as *const c_void,
            self.original_ptr as *mut c_void,
            self.bytes_stolen,
        );

        VirtualProtect(
            self.original_ptr,
            self.bytes_stolen,
            old_prot,
            &mut old_prot,
        )?;

        VirtualFree(self.original as *mut c_void, 0, MEM_RELEASE)?;

        Ok(())
    }
}

unsafe fn set_up_trampoline(
    to_hook: *const c_void,
    hook_len: usize,
) -> Result<*mut c_void, HookError> {
    let tramp = VirtualAlloc(None, hook_len + JMP_SIZE, MEM_COMMIT, PAGE_READWRITE) as *mut u8;
    if tramp.is_null() {
        return Err(HookError::MemAlloc);
    }

    ptr::copy_nonoverlapping(to_hook, tramp as *mut c_void, hook_len);

    let jmp_from = tramp.add(hook_len);
    let jmp_to = to_hook.byte_add(hook_len);
    place_jmp(jmp_from as *mut c_void, jmp_to, JMP_SIZE);

    let mut old_prot: PAGE_PROTECTION_FLAGS = Default::default();
    VirtualProtect(
        tramp as *const c_void,
        hook_len + JMP_SIZE,
        PAGE_EXECUTE_READ,
        &mut old_prot,
    )?;

    Ok(tramp as *mut c_void)
}

fn determine_hook_length(to_hook: *const c_void) -> Result<usize, HookError> {
    let code = unsafe {
        std::slice::from_raw_parts(to_hook as *const u8, JMP_SIZE + X86_MAX_FUNCTION_SIZE)
    };

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;

    let insts = cs.disasm_all(code, to_hook as u64)?;

    ensure_no_relative_addr(&cs, &insts)?;

    let mut length = 0;
    for i in insts.iter() {
        length += i.bytes().len();
        if length >= JMP_SIZE {
            break;
        }
    }
    Ok(length)
}

fn ensure_no_relative_addr(cs: &Capstone, insts: &Instructions) -> Result<(), HookError> {
    for i in insts.iter() {
        let detail = cs.insn_detail(i)?;
        let arch_detail = detail.arch_detail();
        let arch_detail_x86 = arch_detail
            .x86()
            .expect("This shouldn't ever be compiled for anything but x86");
        for operand in arch_detail_x86.operands() {
            match operand.op_type {
                X86OperandType::Imm(_) | X86OperandType::Mem(_) => return Err(HookError::Relative),
                _ => (),
            }
        }
    }
    Ok(())
}

unsafe fn place_jmp(to_hook: *mut c_void, jmp_to: *const c_void, hook_len: usize) {
    ptr::write_bytes(to_hook, 0x90, hook_len);

    let to_hook_b = to_hook as *mut u8;

    // mov rax, <x64-abs>
    ptr::write_unaligned(to_hook_b, 0x48);
    ptr::write_unaligned(to_hook_b.add(1), 0xB8);
    ptr::write_unaligned(to_hook_b.add(2) as *mut usize, jmp_to as usize);

    // jmp rax
    ptr::write_unaligned(to_hook.add(10) as *mut u16, 0xE0FF);
}

unsafe fn place_jmp_protected(
    to_hook: *const c_void,
    jmp_to: *const c_void,
    hook_len: usize,
) -> Result<(), HookError> {
    let mut old_prot: PAGE_PROTECTION_FLAGS = Default::default();

    VirtualProtect(to_hook, hook_len, PAGE_EXECUTE_READWRITE, &mut old_prot)?;

    place_jmp(to_hook as *mut c_void, jmp_to, hook_len);

    VirtualProtect(to_hook, hook_len, old_prot, &mut old_prot)?;

    Ok(())
}

pub unsafe fn hook<O, H>(to_hook: *const O, hook_fn: *const H) -> Result<Hooked<O>, HookError> {
    let to_hook_ptr = to_hook as *const c_void;
    let hook_fn_ptr = hook_fn as *const c_void;

    let hook_len = determine_hook_length(to_hook_ptr)?;

    let tramp = set_up_trampoline(to_hook_ptr, hook_len)?;

    place_jmp_protected(to_hook_ptr, hook_fn_ptr, hook_len)?;

    Ok(Hooked {
        original: &*(tramp as *const O),
        original_ptr: to_hook as *const c_void,
        _hook_fn: hook_fn as *const O,
        bytes_stolen: hook_len,
    })
}
