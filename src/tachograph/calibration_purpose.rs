use crate::impl_enum_from_u8;

#[derive(Debug)]
pub enum CalibrationPurpose {
    Reserved = 0,
    Activation = 1,
    FirstInstallation = 2,
    Installation = 3,
    PeriodicInspection = 4,
    VRNEntryByCompany = 5,
    TimeAdjustmentWithoutCalibration = 6,
    Unknown = 7,
}

impl_enum_from_u8!(
    CalibrationPurpose {
        Reserved = 0,
        Activation = 1,
        FirstInstallation = 2,
        Installation = 3,
        PeriodicInspection = 4,
        VRNEntryByCompany = 5,
        TimeAdjustmentWithoutCalibration = 6,
        Unknown = 7,
    }
);
