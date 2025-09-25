use serde::Serialize;

use crate::impl_enum_from_u8;

/// Indicates the status of a tachograph card, specifically whether it is inserted or removed from a card slot.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum CardStatus {
    /// The card is currently inserted in a card slot.
    Inserted = 0,
    /// The card has been removed from the card slot.
    Removed = 1,
    /// The card status is unknown.
    Unknown = 255,
}

impl_enum_from_u8!(
    CardStatus {
        Inserted = 0,
        Removed = 1,
        Unknown = 255
    }
);
