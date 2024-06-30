use std::ffi::c_void;
use std::ptr;

use crate::cheese::classes::ped::{CPed, CPedPtr};
use crate::cheese::main::PROC;
use crate::cheese::mem::signatures::SignatureError;

pub const OFF_M_LOCAL_PLAYER: usize = 0x8;

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

#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CPedFactory {
    pub vtable: *const c_void,
    local_player: *mut CPed,
}

const FACTORY_INSTANCE: &str = "48 8B 05 ?? ?? ?? ?? 33 D2 48 8B 40 08";
const FACTORY_INSTANCE_OFFSETS: [usize; 2] = [3, 7];

static mut INSTANCE: *mut CPedFactoryPtr = ptr::null_mut();

pub struct CPedFactoryPtr(usize);

impl CPedFactory {
    pub unsafe fn init() -> Result<(), SignatureError> {
        INSTANCE =
            PROC.scan_sig(FACTORY_INSTANCE, &FACTORY_INSTANCE_OFFSETS)? as *mut CPedFactoryPtr;
        Ok(())
    }
    pub unsafe fn get_instance() -> Option<CPedFactoryPtr> {
        PROC.read(INSTANCE as usize).map(|p| CPedFactoryPtr(p))
    }
    pub unsafe fn get_local_player() -> Option<CPedPtr> {
        Self::get_instance().map(|fac| fac.local_player())?
    }
}

impl CPedFactoryPtr {
    pub unsafe fn local_player(&self) -> Option<CPedPtr> {
        let local = CPedPtr(PROC.read::<usize>(self.0 + OFF_M_LOCAL_PLAYER)?);
        Some(local).filter(|x| x.0 != 0)
    }
}
