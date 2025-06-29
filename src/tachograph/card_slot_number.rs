#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum CardSlotNumber {
    Driver,
    CoDriver,
    Unknown = 255,
}
