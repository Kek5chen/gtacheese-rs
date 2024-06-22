use crate::cheese::classes::math::Vector4;
use crate::cheese::classes::player_info::CPlayerInfo;

#[repr(C, align(8))]
pub struct CPed {
    m1280: [f32; 4],
    gap10: [u8; 16],
    qword20: u64,
    byte28: u8,
    gap29: [u8; 103],
    ms_playerFallbackPos: Vector4,
    gapA0: [u8; 32],
    dwordC0: u32,
    gapC4: [u8; 12],
    qwordD0: u64,
    gapD8: [u8; 176],
    byte188: u8,
    gap189: [u8; 167],
    dword230: u32,
    gap234: [u8; 76],
    health: f32,
    gap284: [u8; 2996],
    charE38: i8,
    gapE39: [u8; 599],
    byte1090: u8,
    gap1091: [u8; 15],
    qword10A0: u64,
    m_PlayerInfo: *mut CPlayerInfo,
    gap10B0: [u8; 48],
    qword10E0: u64,
    gap10E8: [u8; 100],
    dword114C: u32,
    gap1150: [u8; 76],
    dword119C: u32,
    gap11A0: [u8; 680],
    m_PedConfigFlags: u32,
    dword144C: u32,
    gap1450: [u8; 8],
    dword1458: u32,
    gap145C: [u8; 20],
    dword1470: u32,
    gap1474: [u8; 8],
    dword147C: u32,
    gap1480: [u8; 76],
    dword14CC: u32,
    gap14D0: [u8; 72],
    float1518: f32,
    float151C: f32,
    gap1520: [u8; 16],
    m_pMyVehicle: u64,
}

impl CPed {
    pub unsafe fn player_info(&self) -> Option<&'static mut CPlayerInfo> {
        self.m_PlayerInfo.as_mut()
    }
}