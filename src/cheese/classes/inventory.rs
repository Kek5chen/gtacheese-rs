use crate::cheese::classes::ammo_repository::CAmmoItemRepositoryPtr;
use crate::cheese::classes::ped::CPed;
use crate::cheese::main::PROC;

const OFF_M_AMMO_REPOSITORY: usize = 0x48;
pub struct CInventory;

pub struct CInventoryPtr(pub usize);

impl CInventory {
    pub unsafe fn get_local_inventory() -> Option<CInventoryPtr> {
        CPed::local_player().and_then(|local| local.get_inventory())
    }
}

impl CInventoryPtr {
    pub unsafe fn get_ammo_repositoy(&self) -> CAmmoItemRepositoryPtr {
        CAmmoItemRepositoryPtr(self.0 + OFF_M_AMMO_REPOSITORY)
    }
}
