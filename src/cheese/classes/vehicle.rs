use crate::cheese::classes::physical::CPhysicalPtr;

pub struct CVehicle;

pub struct CVehiclePtr(pub usize);

impl CVehiclePtr {
    pub fn up(&self) -> CPhysicalPtr {
        CPhysicalPtr(self.0)
    }
}