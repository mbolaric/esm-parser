pub const UNKNOWN: u8 = 0xFF;

pub const VU_HEADER_MAGIC_NUMBER: u8 = 0x76;

pub const VU_HEADER_G1: [u8; 2] = [0x76, 0x1];
pub const VU_HEADER_G2: [u8; 2] = [0x76, 0x21]; 
pub const VU_HEADER_G2_V2: [u8; 2] = [0x76, 0x31];

pub const CARD_HEADER: [u8; 2] = [0x00, 0x2];
pub const CARD_HEADER_VU_DATA: [u8; 2] = [0x76, 0x6];

pub const MINIMUM_G2_CARD_DATA_LENGTH: u64 = 30000;