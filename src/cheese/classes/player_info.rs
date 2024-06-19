use crate::cheese::classes::wanted::CWanted;

#[allow(unused_variables, non_snake_case)]
#[repr(C, align(4))]
pub struct CPlayerInfo {
    m1280: [f32; 4], // was __m128
    gap10: [u8; 244],
    cwanted104: CWanted,
    gap5EC: [u8; 564],
    pub m_Wanted: CWanted,
    gapD08: [u8; 60],
    floatD44: f32,
    floatD48: f32,
    floatD4C: f32,
    dwordD50: u32,
    gapD54: [u8; 592],
    dwordFA4: u32,
    gapFA8: [u8; 52],
    dwordFDC: u32,
    dwordFE0: u32,
    dwordFE4: u32,
    gapFE8: [u8; 106],
    byte1052: u8,
    gap1053: [u8; 9],
    float105C: f32,
    dword1060: u32,
}