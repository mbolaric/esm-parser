#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum DrivingStatus {
    Unknown = 255,
    SingleOrUnknown = 0,
    CrowOrKnown = 1,
}
