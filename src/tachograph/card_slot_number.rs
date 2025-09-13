use serde::Serialize;

use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum CardSlotNumber {
    Driver = 0,
    CoDriver = 1,
    Unknown = 255,
}

impl_enum_from_u8!(
    CardSlotNumber {
        Driver = 0,
        CoDriver = 1,
        Unknown = 255,
    }
);
