use std::fmt::Display;

use crate::impl_enum_from_u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CardFileID {
    Unknown = 0,
    ICC = 2,
    IC = 5,
    Tachograph = 0x500,
    ApplicationIdentification = 0x501,
    ApplicationIdentificationV2 = 0x525,
    EventsData = 0x502,
    FaultsData = 0x503,
    DriverActivityData = 0x504,
    VehiclesUsed = 0x505,
    Places = 0x506,
    CurrentUsage = 0x507,
    ControlActivityData = 0x508,
    Calibration = 0x50A,
    SensorInstallationData = 0x50B,
    ControllerActivityData = 0x50C,
    CardDownload = 0x50E,
    Identification = 0x520,
    DrivingLicenseInfo = 0x521,
    SpecificConditions = 0x522,
    VehicleUnitsUsed = 0x523,
    GnssPlaces = 0x524,
    BorderCrossings = 0x528,
    CalibrationAddData = 0x531,
    VUConfiguration = 0x540,
    CardCertificate = 0xC100,
    CardSignCertificate = 0xC101,
    CACertificate = 0xC108,
    LinkCertificate = 0xC109,
    MF = 0x3F00,
}

impl Display for CardFileID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_enum_from_u16!(
    CardFileID {
        Unknown = 0,
        IC = 5,
        ICC = 2,
        Tachograph = 0x500,
        ApplicationIdentification = 0x501,
        EventsData = 0x502,
        FaultsData = 0x503,
        DriverActivityData = 0x504,
        VehiclesUsed = 0x505,
        Places = 0x506,
        CurrentUsage = 0x507,
        ControlActivityData = 0x508,
        Calibration = 0x50A,
        SensorInstallationData = 0x50B,
        ControllerActivityData = 0x50C,
        CardDownload = 0x50E,
        Identification = 0x520,
        DrivingLicenseInfo = 0x521,
        SpecificConditions = 0x522,
        VehicleUnitsUsed = 0x523,
        GnssPlaces = 0x524,
        CardCertificate = 0xC100,
        CardSignCertificate = 0xC101,
        CACertificate = 0xC108,
        LinkCertificate = 0xC109,
        MF = 0x3F00,
    }
);
