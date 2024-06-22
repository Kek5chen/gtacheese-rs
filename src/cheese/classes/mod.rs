
use std::collections::HashMap;
use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::vehicle_factory::CVehicleFactory;
use crate::cheese::mem::signatures::SignatureError;

#[allow(unused)]
pub mod ped_factory;
#[allow(unused)]
pub mod ped;
#[allow(unused)]
pub mod wanted;
#[allow(unused)]
pub mod player_info;
#[allow(unused)]
pub mod math;
pub mod vehicle_factory;

pub unsafe fn init_classes() -> HashMap<&'static str, Result<(), SignatureError>> {
    let mut results = HashMap::new();
    results.insert("CPedFactory", CPedFactory::init());
    results.insert("CVehicleFactory", CVehicleFactory::init());
    results
}