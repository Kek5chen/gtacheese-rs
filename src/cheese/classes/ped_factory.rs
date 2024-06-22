use crate::cheese::classes::ped::CPed;
use crate::cheese::mem::signatures::{scan_sig, SignatureError};
use std::ffi::c_void;
use std::ptr;
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
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

pub type CreatePedFn = unsafe extern "C" fn(
    this: *mut CPedFactory,
    ped_control_info: &CControlledByInfo,
    model_id: u32,
    p_mat: &[[f32; 3]; 4],
    apply_default_variation: bool,
    should_be_cloned: bool,
    created_by_script: bool,
    fail_silent_if_out_of_peds: bool,
    scenario_ped_created_by_concealed_player: bool,
) -> &'static mut CPed;

pub type ClonePedFn = unsafe extern "C" fn(
    this: *mut CPedFactory,
    source: *const CPed,
    b_register_as_network_object: bool,
    b_link_blends: bool,
    b_clone_compressed_damage: bool,
) -> &'static mut CPed;

#[allow(unknown_lints)]
#[allow(type_complexity)]
#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CPedFactoryVTable {
    pub __DESTRUCTOR: extern "fastcall" fn(this: *mut CPedFactory),
    pub CreatePed: CreatePedFn,
    pub CreatePedFromSource: TooLazyToDefineThisFn,
    pub ClonePed: ClonePedFn,
}

#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CPedFactory {
    pub vtable: &'static CPedFactoryVTable,
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
        (self.vtable.ClonePed)(self, source, b_register_as_network_object, b_link_blends, b_clone_compressed_damage)
    }
}

