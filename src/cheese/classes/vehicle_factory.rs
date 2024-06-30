use std::ptr;

use crate::cheese::main::PROC;
use crate::cheese::mem::signatures::SignatureError;

const VEHICLE_FACTORY_SIG: &str = "48 8B 3D ?? ?? ?? ?? 8B 96 40 03 00 00 48 8B 07 48 8B CF";
const VEHICLE_FACTORY_OFFSETS: [usize; 2] = [3, 7];

// TODO: Import Type

pub struct CVehicleFactory {}
#[derive(Copy, Clone)]
pub struct CVehicleFactoryPtr(pub usize);

static mut INSTANCE: *mut CVehicleFactoryPtr = ptr::null_mut();

impl CVehicleFactory {
    pub unsafe fn init() -> Result<(), SignatureError> {
        INSTANCE = PROC.scan_sig(VEHICLE_FACTORY_SIG, &VEHICLE_FACTORY_OFFSETS)?
            as *mut CVehicleFactoryPtr;
        Ok(())
    }

    pub unsafe fn get_instance() -> Option<CVehicleFactoryPtr> {
        PROC.read(INSTANCE as usize)
            .map(|fac| CVehicleFactoryPtr(fac))
    }
}
