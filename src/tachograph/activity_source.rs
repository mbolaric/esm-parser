use serde::Serialize;

/// Represents the source of a driver's activity, indicating whether it was
/// recorded automatically by the VU or entered manually by the driver.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum ActivitySource {
    /// The activity was recorded automatically by the Vehicle Unit.
    Automatic,
    /// The activity was entered manually by the driver.
    Manual,
    /// The source of the activity is unknown.
    Unknown,
}
