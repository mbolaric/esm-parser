use serde::Serialize;

/// Represents the driving status, indicating whether the vehicle is being operated by a single driver or a crew.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum DrivingStatus {
    /// The driving status is unknown.
    Unknown = 255,
    /// The vehicle is operated by a single driver, or the status is not known.
    SingleOrUnknown = 0,
    /// The vehicle is operated by a crew (driver and co-driver).
    CrowOrKnown = 1,
}
