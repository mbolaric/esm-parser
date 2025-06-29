use crate::impl_enum_from_u16;

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum ActivityType {
    Rest = 0,
    Availability = 1,
    Work = 2,
    Driving = 3,
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
