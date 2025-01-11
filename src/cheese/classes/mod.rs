use std::collections::HashMap;

use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::vehicle_factory::CVehicleFactory;
use crate::cheese::mem::signatures::SignatureError;

// TOO UNSTABLE: include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[allow(unused)]
pub mod math;
#[allow(unused)]
pub mod ped;
#[allow(unused)]
pub mod ped_factory;
#[allow(unused)]
pub mod player_info;
pub mod vehicle_factory;
#[allow(unused)]
pub mod wanted;
pub mod inventory;
pub mod ammo_repository;
pub mod weapon_repository;
mod physical;
mod vehicle;

pub unsafe fn init_classes() -> HashMap<&'static str, Result<(), SignatureError>> {
    let mut results = HashMap::new();
    results.insert("CPedFactory", CPedFactory::init());
    results.insert("CVehicleFactory", CVehicleFactory::init());
    results
}
