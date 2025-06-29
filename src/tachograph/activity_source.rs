#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ActivitySource {
    Tacho,
    Manual,
    Unknown,
}
