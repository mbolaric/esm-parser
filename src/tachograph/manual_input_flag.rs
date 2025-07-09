use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ManualInputFlag {
    NoEntry = 0,
    ManualEntries = 1,
    Unknown = 255,
}

impl_enum_from_u8!(
    ManualInputFlag {
        NoEntry = 0,
        ManualEntries = 1,
        Unknown = 255,
    }
);
