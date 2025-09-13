use serde::Serialize;

///Whenever the driver has has changed of driving status
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum DrivingStatus {
    Unknown = 255,
    SingleOrUnknown = 0,
    CrowOrKnown = 1,
}
