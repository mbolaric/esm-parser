use crate::impl_enum_from_u8;

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
