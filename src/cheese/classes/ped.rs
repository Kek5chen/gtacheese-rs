use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::player_info::{CPlayerInfo, CPlayerInfoPtr};
use crate::cheese::main::PROC;

pub const OFF_M_PLAYER_INFO: usize = 0x10A8;
pub const OFF_M_HEALTH: usize = 0x280;
pub const OFF_M_MAX_HEALTH: usize = 0x284;

pub struct CPedPtr(pub(super) usize);

pub struct CPed;
impl CPed {
    pub unsafe fn local_player() -> Option<CPedPtr> {
        CPedFactory::get_local_player()
    }
}

impl CPedPtr {
    pub unsafe fn player_info(&self) -> Option<CPlayerInfo> {
        PROC.read::<CPlayerInfo>(self.0 + OFF_M_PLAYER_INFO)
    }

    pub unsafe fn set_seatbelt(&self, on: bool) -> anyhow::Result<()> {
        let byte = match on {
            true => 0xC9,
            false => 0xC8,
        };
        Ok(PROC.write(self.0 + 0x143C, byte)?)
    }
    pub unsafe fn get_max_health(&self) -> Option<f32> {
        PROC.read::<f32>(self.0 + OFF_M_HEALTH)
    }

    pub unsafe fn get_health(&self) -> Option<f32> {
        PROC.read::<f32>(self.0 + OFF_M_MAX_HEALTH)
    }

    pub unsafe fn set_health(&self, health: f32) -> anyhow::Result<()> {
        Ok(PROC.write(self.0 + OFF_M_HEALTH, health)?)
    }

    pub unsafe fn kill(&self) -> anyhow::Result<()> {
        Ok(PROC.write(self.0 + OFF_M_HEALTH, 0)?)
    }
    pub unsafe fn get_player_info(&self) -> Option<CPlayerInfoPtr> {
        PROC.read::<CPlayerInfoPtr>(self.0 + OFF_M_PLAYER_INFO)
    }
}
