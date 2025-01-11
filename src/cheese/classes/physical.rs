use crate::cheese::main::PROC;

pub const OFF_M_HEALTH: usize = 0x280;
pub const OFF_M_MAX_HEALTH: usize = 0x284;

pub struct CPhysical;
pub struct CPhysicalPtr(pub usize);

impl CPhysicalPtr {
    pub unsafe fn get_max_health(&self) -> Option<f32> {
        PROC.read::<f32>(self.0 + OFF_M_MAX_HEALTH)
    }

    pub unsafe fn get_health(&self) -> Option<f32> {
        PROC.read::<f32>(self.0 + OFF_M_HEALTH)
    }

    pub unsafe fn set_health(&self, health: f32) -> anyhow::Result<()> {
        Ok(PROC.write(self.0 + OFF_M_HEALTH, health)?)
    }

    pub unsafe fn kill(&self) -> anyhow::Result<()> {
        Ok(PROC.write(self.0 + OFF_M_HEALTH, 0)?)
    }
}