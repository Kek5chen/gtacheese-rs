use std::ffi::c_void;
use std::ptr;
use crate::cheese::main::PROC;
use crate::cheese::mem::signatures::SignatureError;

const VEHICLE_FACTORY_SIG: &str = "48 8B 3D ?? ?? ?? ?? 8B 96 40 03 00 00 48 8B 07 48 8B CF";
const VEHICLE_FACTORY_OFFSETS: [usize; 2] = [3, 7];

pub type CreateVehicleFn = unsafe extern "C" fn(
    this: *const c_void,
    modelId: u32,
    ownedBy: u8,
    popType: u32,
    pMat: *const [[f32; 4]; 3],
    bClone: bool,
    bCreateAsInactive: bool,
) -> &'static u8;

#[allow(unknown_lints)]
#[allow(type_complexity)]
#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CVehicleFactoryVTable {
    pub __DESTRUCTOR: extern "fastcall" fn(this: *mut CVehicleFactory),
    pub Create: CreateVehicleFn,
}

#[allow(unused_variables, non_snake_case)]
#[repr(C)]
pub struct CVehicleFactory {
    pub vtable: &'static CVehicleFactoryVTable,
}
static mut INSTANCE: *mut *mut CVehicleFactory = ptr::null_mut();

impl CVehicleFactory {
    pub unsafe fn init() -> Result<(), SignatureError> {
        INSTANCE = PROC.scan_sig(VEHICLE_FACTORY_SIG, &VEHICLE_FACTORY_OFFSETS)?;
        Ok(())
    }

    pub unsafe fn get_instance() -> Option<&'static mut CVehicleFactory> {
        (*INSTANCE).as_mut()
    }
}