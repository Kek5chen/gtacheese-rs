use crate::cheese::classes::ped::OFF_M_PLAYER_INFO;
use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::wanted::CWantedPtr;
use crate::cheese::main::PROC;

pub const OFF_M_WANTED: usize = 0x830;

pub struct CPlayerInfo;

impl CPlayerInfo {
    pub unsafe fn get_local_player_info() -> Option<CPlayerInfoPtr> {
        CPedFactory::get_local_player()
            .map(|p| {
                PROC.read::<usize>(p.0 + OFF_M_PLAYER_INFO)
                    .map(|info| CPlayerInfoPtr(info))
            })
            .flatten()
    }
}
pub struct CPlayerInfoPtr(pub usize);

impl CPlayerInfoPtr {
    pub unsafe fn get_wanted(&self) -> CWantedPtr {
        CWantedPtr(self.0 + OFF_M_WANTED)
    }
}
