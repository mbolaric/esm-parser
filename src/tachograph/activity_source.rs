#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ActivitySource {
    Automatic,
    Manual,
    Unknown,
}
