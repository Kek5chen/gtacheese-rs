use crate::cheese::classes::ped_factory::CPedFactory;

#[repr(C, align(8))]
pub struct CWanted {
    gap0: [u8; 32],
    pub m_nWantedLevel: u32,
    dword28: u32,
    pub m_TimeWhenNewWantedLevelTakesEffect: [u8; 24],
    pub m_nMaxCopCarsInPursuit: u8,
    gap45: [u8; 67],
    dword88: u32,
    gap8C: [u8; 20],
    wordA0: u16,
    byteA2: u8,
    gapA3: [u8; 21],
    pub m_WantedLevel: i32,
    pub m_nNewWantedLevel: u32,
    gapC4: [u8; 804],
    dword3E8: u32,
    qword3EC: u64,
    gap3F4: [u8; 12],
    dword400: u32,
    pub m_nMaxCopsInPursuit: u8,
    gap405: [u8; 188],
    byte4C1: u8,
    dword4C8: u32,
    float4CC: f32,
    gap4D0: [u8; 4],
    dword4D4: u32,
    gap4D8: [u8; 8],
    pqword4E0: *mut u64,
}

impl CWanted {
}