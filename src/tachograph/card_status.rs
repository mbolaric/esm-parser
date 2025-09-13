use crate::impl_enum_from_u8;

///Whenever the driver has inserted or withdrawn his card.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum CardStatus {
    Inserted = 0,
    Removed = 1,
    Unknown = 255,
}

impl_enum_from_u8!(
    CardStatus {
        Inserted = 0,
        Removed = 1,
        Unknown = 255
    }
);
