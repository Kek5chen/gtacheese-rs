use crate::cheese::classes::player_info::CPlayerInfo;
use crate::cheese::main::PROC;

pub const OFF_M_N_WANTED_LEVEL: usize = 0x20;
pub const OFF_M_WANTED_LEVEL: usize = 0xB8;

pub struct CWanted;
impl CWanted {
    pub unsafe fn get_local_wanted() -> Option<CWantedPtr> {
        CPlayerInfo::get_local_player_info().map(|info| info.get_wanted())
    }
}

pub struct CWantedPtr(pub usize);

impl CWantedPtr {
    pub unsafe fn get_wanted_level(&self) -> Option<u32> {
        PROC.read::<u32>(self.0 + OFF_M_WANTED_LEVEL)
    }

    fn convert_wanted_level(level: u32) -> u32 {
        match level {
            0 => 0,
            1 => 50,
            2 => 180,
            3 => 550,
            4 => 1200,
            5 => 3800,
            _ => 5000, // technically 5700 is max
        }
    }
    pub unsafe fn set_wanted_level(&self, new_level: u32) -> anyhow::Result<()> {
        PROC.write(self.0 + OFF_M_WANTED_LEVEL, new_level)?;
        PROC.write(
            self.0 + OFF_M_N_WANTED_LEVEL,
            Self::convert_wanted_level(new_level),
        )?;
        Ok(())
    }
}
