use serde::Serialize;

use crate::impl_enum_from_u8;

/// Code explaining why a set of calibration parameters was recorded. This
// data type is related to Annex 1B requirements 097 and 098 and
// Annex 1C requirements 119.
#[derive(Debug, Serialize)]
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
