/// Magic number indicating the start of a Vehicle Unit (VU) data block.
pub const VU_HEADER_MAGIC_NUMBER: u8 = 0x76;

/// Header for Generation 1 (G1) Vehicle Unit (VU) data.
pub const VU_HEADER_G1: [u8; 2] = [0x76, 0x1];
/// Header for Generation 2 (G2) Vehicle Unit (VU) data.
pub const VU_HEADER_G2: [u8; 2] = [0x76, 0x21];
/// Header for Generation 2, Version 2 (G2v2) Vehicle Unit (VU) data.
pub const VU_HEADER_G2_V2: [u8; 2] = [0x76, 0x31];

/// Header for card data.
pub const CARD_HEADER: [u8; 2] = [0x00, 0x2];
/// Header for card data embedded within a Vehicle Unit (VU) data file.
pub const CARD_HEADER_VU_DATA: [u8; 2] = [0x76, 0x6];

/// Minimum expected data length for a Generation 2 (G2) card, used for validation.
pub const MINIMUM_G2_CARD_DATA_LENGTH: u64 = 30000;
