use serde::Serialize;

use crate::impl_enum_from_u8;

/// Code identifying whether a cardholder has manually entered driver
/// activities at card insertion or not (Annex 1B requirement 081 and
/// Annex 1C requirement 102).
#[derive(Debug, PartialEq, Serialize)]
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
