use core::fmt;

use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum EquipmentType {
    Reserved = 0,
    DriverCard = 1,
    WorkshopCard = 2,
    ControlCard = 3,
    CompanyCard = 4,
    ManufacturingCard = 5,
    VehicleUnit = 6,
    MotionSensor = 7,
    GnssFacility = 8,
    RemoteCommunicationFacility = 9,
    ITSInterfaceModule = 10,
    Plaque = 11,
    M1N1Adapter = 12,
    EuropeanRootCA = 13,
    MemberStateCA = 14,
    ExternalGnssConnetion = 15,
    Unused = 16,
    DriverKey = 170,
    Unknown = 254,
    NullCard = 255,
}

impl fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_enum_from_u8!(
    EquipmentType {
        Reserved = 0,
        DriverCard = 1,
        WorkshopCard = 2,
        ControlCard = 3,
        CompanyCard = 4,
        ManufacturingCard = 5,
        VehicleUnit = 6,
        MotionSensor = 7,
        GnssFacility = 8,
        RemoteCommunicationFacility = 9,
        ITSInterfaceModule = 10,
        Plaque = 11,
        M1N1Adapter = 12,
        EuropeanRootCA = 13,
        MemberStateCA = 14,
        ExternalGnssConnetion = 15,
        Unused = 16,
        DriverKey = 170,
        Unknown = 254,
        NullCard = 255,
    }
);
