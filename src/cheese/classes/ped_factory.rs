use crate::cheese::classes::ped::CPed;
use crate::cheese::mem::signatures::{scan_sig, SignatureError};
use std::ffi::{c_void, CString};
use std::{mem, ptr};
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use crate::cheese::mem::hook::hook;
use crate::util::MessageBox;

#[derive(Debug)]
#[allow(unused_variables, non_snake_case)]
pub struct CControlledByInfo {
    flags: u8,
}

impl CControlledByInfo {
    const IS_CONTROLLED_BY_NETWORK: u8 = 0b00000001;
    const IS_PLAYER: u8 = 0b00000010;

    pub fn new(is_network: bool, is_player: bool) -> Self {
        let mut flags = 0;
        if is_network {
            flags |= Self::IS_CONTROLLED_BY_NETWORK;
        }
        if is_player {
            flags |= Self::IS_PLAYER;
        }
        Self { flags }
    }

    pub fn set(&mut self, rhs: &CControlledByInfo) {
        self.flags = rhs.flags;
    }

    pub fn is_controlled_by_network(&self) -> bool {
        (self.flags & Self::IS_CONTROLLED_BY_NETWORK) != 0
    }

    pub fn is_controlled_by_local_ai(&self) -> bool {
        (self.flags & Self::IS_CONTROLLED_BY_NETWORK) == 0 && (self.flags & Self::IS_PLAYER) == 0
    }

    pub fn is_controlled_by_network_ai(&self) -> bool {
        (self.flags & Self::IS_CONTROLLED_BY_NETWORK) != 0 && (self.flags & Self::IS_PLAYER) == 0
    }

    pub fn is_controlled_by_local_or_network_ai(&self) -> bool {
        (self.flags & Self::IS_PLAYER) == 0
    }

    pub fn is_controlled_by_local_player(&self) -> bool {
        (self.flags & Self::IS_CONTROLLED_BY_NETWORK) == 0 && (self.flags & Self::IS_PLAYER) != 0
    }

    pub fn is_controlled_by_network_player(&self) -> bool {
        (self.flags & Self::IS_CONTROLLED_BY_NETWORK) != 0 && (self.flags & Self::IS_PLAYER) != 0
    }

    pub fn is_controlled_by_local_or_network_player(&self) -> bool {
        (self.flags & Self::IS_PLAYER) != 0
    }
}

type TooLazyToDefineThisFn = *const c_void;

type CreatePedFn = unsafe extern "fastcall" fn(
    this: *mut CPedFactory,
    ped_control_info: &CControlledByInfo,
    model_id: u32,
    p_mat: [[f32; 3]; 4],
    apply_default_variation: bool,
    should_be_cloned: bool,
    created_by_script: bool,
    fail_silent_if_out_of_peds: bool,
    scenario_ped_created_by_concealed_player: bool,
) -> &'static mut CPed;

type ClonePedFn = unsafe extern "fastcall" fn(
    this: *mut CPedFactory,
    source: *const CPed,
    b_register_as_network_object: bool,
    b_link_blends: bool,
    b_clone_compressed_damage: bool,
) -> &'static mut CPed;

#[allow(type_complexity)]
#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CPedFactoryVTable {
    DESTRUCTOR: extern "fastcall" fn(this: *mut CPedFactory),
    CreatePed: CreatePedFn,
    CreatePedFromSource: TooLazyToDefineThisFn,
    ClonePed: ClonePedFn,
}

#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CPedFactory {
    vtable: &'static CPedFactoryVTable,
    local_player: *mut CPed,
}

const FACTORY_INSTANCE: &str = "48 8B 05 ?? ?? ?? ?? 33 D2 48 8B 40 08";
const FACTORY_INSTANCE_OFFSETS: [usize; 2] = [3, 7];

static mut INSTANCE: *mut *mut CPedFactory = ptr::null_mut();

impl CPedFactory {
    pub unsafe fn init() -> Result<(), SignatureError> {
        INSTANCE = scan_sig(FACTORY_INSTANCE, &FACTORY_INSTANCE_OFFSETS)?;
        Ok(())
    }
    pub unsafe fn get_instance() -> Option<&'static mut CPedFactory> {
        (*INSTANCE).as_mut()
    }

    pub unsafe fn local_player<'a, 'b>(&'a self) -> Option<&'b mut CPed> {
        self.local_player.as_mut()
    }
    
    pub unsafe fn clone_ped(&mut self,
                            source: *const CPed,
                            b_register_as_network_object: bool,
                            b_link_blends: bool,
                            b_clone_compressed_damage: bool,
    )
        -> &'static CPed
    {
        log::info!("CPedFactory: {:?}", self as *const CPedFactory);
        log::info!("VTable: {:?}", self.vtable as *const CPedFactoryVTable);
        log::info!("ClonePed: {:?}", self.vtable.ClonePed);

        MessageBox("I'm waiting here again.", "Waiting.", MB_OK);
        
        let name = CString::new("MessageBoxA").unwrap();
        let dll = CString::new("kernel32.dll").unwrap();
        let mboxa = GetProcAddress(GetModuleHandleA(PCSTR(dll.as_ptr() as *const u8)).unwrap(), PCSTR(name.as_ptr() as *const u8));

        let hooked = hook(mem::transmute::<_, &unsafe extern "C" fn(u32, usize, usize, u32) -> u32>(mboxa), &MessageBoxAHooked).unwrap();
        
        MessageBox("TestBox", "Does this still do somethin?.", MB_OK);

        hooked.free().unwrap();
        
        
        mem::transmute(0usize)
        // (self.vtable.ClonePed)(self, source, b_register_as_network_object, b_link_blends, b_clone_compressed_damage)
    }
}

pub unsafe extern "C" fn MessageBoxAHooked(hwnd: u32, lptext: usize, lpcaption: usize, utype: u32) -> u32 {
    log::info!("We're in boys");
    
    0
}


unsafe extern "C" fn CreatePedHook(
    this: *mut CPedFactory,
    ped_control_info: &CControlledByInfo,
    model_id: u32,
    p_mat: [[f32; 3]; 4],
    apply_default_variation: bool,
    should_be_cloned: bool,
    created_by_script: bool,
    fail_silent_if_out_of_peds: bool,
    scenario_ped_created_by_concealed_player: bool,
) -> &'static mut CPed {
    log::info!("this: {:?}", this);
    log::info!("ped_control_info: {ped_control_info:?}");
    log::info!("model_id: {model_id}");
    log::info!("p_mat: {p_mat:?}");
    log::info!("apply_default_variation: {apply_default_variation}");
    log::info!("should_be_cloned: {should_be_cloned}");

    mem::transmute(0usize)
}
