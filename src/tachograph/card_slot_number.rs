use serde::Serialize;

use crate::impl_enum_from_u8;

/// Represents the card slot in the Vehicle Unit, distinguishing between the driver and co-driver slots.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum CardSlotNumber {
    /// The slot designated for the main driver.
    Driver = 0,
    /// The slot designated for the co-driver.
    CoDriver = 1,
    /// The card slot is unknown or not applicable.
    Unknown = 255,
}

impl_enum_from_u8!(
    CardSlotNumber {
        Driver = 0,
        CoDriver = 1,
        Unknown = 255,
    }
);
