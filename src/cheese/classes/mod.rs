use std::collections::HashMap;
use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::mem::signatures::SignatureError;

pub mod ped_factory;
pub mod ped;
pub mod wanted;
pub mod player_info;
pub mod math;

pub unsafe fn init_classes() -> HashMap<&'static str, Result<(), SignatureError>> {
    let mut results = HashMap::new();
    results.insert("CPedFactory", CPedFactory::init());
    results
}