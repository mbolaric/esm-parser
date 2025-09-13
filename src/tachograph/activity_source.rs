use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum ActivitySource {
    Automatic,
    Manual,
    Unknown,
}
