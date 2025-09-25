use serde::Serialize;

use crate::impl_enum_from_u16;

/// Represents the type of activity a driver is engaged in, such as driving, working, or resting.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u16)]
pub enum ActivityType {
    /// The driver is on a break or resting.
    Rest = 0,
    /// The driver is available for work but not actively working (e.g., waiting).
    Availability = 1,
    /// The driver is performing work other than driving.
    Work = 2,
    /// The driver is driving the vehicle.
    Driving = 3,
    /// The activity type is unknown or not specified.
    Unknown = 255,
}

impl_enum_from_u16!(
    ActivityType {
        Rest = 0,
        Availability = 1,
        Work = 2,
        Driving = 3,
        Unknown = 255,
    }
);
