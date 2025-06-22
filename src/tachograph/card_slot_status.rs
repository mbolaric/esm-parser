use crate::impl_enum_from_u8;

#[derive(Debug)]
#[repr(u8)]
pub enum CardSlotStatusCode {
    Unknown = 0,
    DriverCard = 1,
    WorkshopCard = 2,
    ControlCard = 3,
    CompanyCard = 4,
}

impl_enum_from_u8!(
    CardSlotStatusCode {
        Unknown = 0,
        DriverCard = 1,
        WorkshopCard = 2,
        ControlCard = 3,
        CompanyCard = 4,
    }
);
