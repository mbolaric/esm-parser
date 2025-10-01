use std::fmt;

use serde::Serialize;

use crate::impl_enum_from_u16;

/// Identifiers for files on a tachograph card.
/// These files are also known as "tacho blocks".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[repr(u16)]
pub enum CardFileID {
    Unknown = 0x00,
    /// Integrated Circuit Card.
    ICC = 0x02,
    /// Integrated Circuit.
    IC = 0x05,
    /// Tachograph application.
    Tachograph = 0x500,
    /// Application Identification.
    ApplicationIdentification = 0x501,
    /// Application Identification (V2).
    ApplicationIdentificationV2 = 0x525,
    /// Events data.
    EventsData = 0x502,
    /// Faults data.
    FaultsData = 0x503,
    /// Driver activity data.
    DriverActivityData = 0x504,
    /// Vehicles used.
    VehiclesUsed = 0x505,
    /// Places.
    Places = 0x506,
    /// Current usage.
    CurrentUsage = 0x507,
    /// Control activity data.
    ControlActivityData = 0x508,
    /// Calibration data.
    Calibration = 0x50A,
    /// Sensor installation data.
    SensorInstallationData = 0x50B,
    /// Controller activity data.
    ControllerActivityData = 0x50C,
    /// Company activity data.
    CompanyActivityData = 0x50D,
    /// Card download information.
    CardDownload = 0x50E,
    /// Card identification data.
    Identification = 0x520,
    /// Driving license information.
    DrivingLicenseInfo = 0x521,
    /// Specific conditions.
    SpecificConditions = 0x522,
    /// Vehicle units used.
    VehicleUnitsUsed = 0x523,
    /// GNSS places.
    GnssPlaces = 0x524,
    /// Border crossings.
    BorderCrossings = 0x528,
    /// Calibration additional data.
    CalibrationAddData = 0x531,
    /// VU configuration.
    VUConfiguration = 0x540,
    /// Card certificate.
    CardCertificate = 0xC100,
    /// Card sign certificate.
    CardSignCertificate = 0xC101,
    /// Certificate Authority certificate.
    CACertificate = 0xC108,
    /// Link certificate.
    LinkCertificate = 0xC109,
    /// Master File.
    MF = 0x3F00,
}

impl fmt::Display for CardFileID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CardFileID::Unknown => "Unknown",
            CardFileID::ICC => "ICC",
            CardFileID::IC => "IC",
            CardFileID::Tachograph => "Tachograph",
            CardFileID::ApplicationIdentification => "ApplicationIdentification",
            CardFileID::ApplicationIdentificationV2 => "ApplicationIdentificationV2",
            CardFileID::EventsData => "EventsData",
            CardFileID::FaultsData => "FaultsData",
            CardFileID::DriverActivityData => "DriverActivityData",
            CardFileID::VehiclesUsed => "VehiclesUsed",
            CardFileID::Places => "Places",
            CardFileID::CurrentUsage => "CurrentUsage",
            CardFileID::ControlActivityData => "ControlActivityData",
            CardFileID::Calibration => "Calibration",
            CardFileID::SensorInstallationData => "SensorInstallationData",
            CardFileID::ControllerActivityData => "ControllerActivityData",
            CardFileID::CompanyActivityData => "CompanyActivityData",
            CardFileID::CardDownload => "CardDownload",
            CardFileID::Identification => "Identification",
            CardFileID::DrivingLicenseInfo => "DrivingLicenseInfo",
            CardFileID::SpecificConditions => "SpecificConditions",
            CardFileID::VehicleUnitsUsed => "VehicleUnitsUsed",
            CardFileID::GnssPlaces => "GnssPlaces",
            CardFileID::BorderCrossings => "BorderCrossings",
            CardFileID::CalibrationAddData => "CalibrationAddData",
            CardFileID::VUConfiguration => "VUConfiguration",
            CardFileID::CardCertificate => "CardCertificate",
            CardFileID::CardSignCertificate => "CardSignCertificate",
            CardFileID::CACertificate => "CACertificate",
            CardFileID::LinkCertificate => "LinkCertificate",
            CardFileID::MF => "MF",
        };
        write!(f, "{}", s)
    }
}

impl_enum_from_u16!(
    CardFileID {
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
        CompanyActivityData = 0x50D,
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
);
