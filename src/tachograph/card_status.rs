use serde::Serialize;

use crate::impl_enum_from_u8;

/// Indicates whether a valid driver or workshop card is inserted in the relevant slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum CardStatus {
    /// A valid driver or workshop card is inserted in the relevant slot.
    Inserted = 0,
    /// No valid driver or workshop card is inserted in the relevant slot.
    NotInserted = 1,
    /// The card status is unknown.
    Unknown = 255,
}

impl_enum_from_u8!(
    CardStatus {
        Inserted = 0,
        NotInserted = 1,
        Unknown = 255
    }
);
