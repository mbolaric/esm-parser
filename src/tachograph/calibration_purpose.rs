use serde::Serialize;

use crate::impl_enum_from_u8;

/// Code explaining why a set of calibration parameters was recorded. This
// data type is related to Annex 1B requirements 097 and 098 and
// Annex 1C requirements 119.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum CalibrationPurpose {
    Reserved = 0,
    Activation = 1,
    FirstInstallation = 2,
    Installation = 3,
    PeriodicInspection = 4,
    VRNEntryByCompany = 5,
    TimeAdjustmentWithoutCalibration = 6,
    /// Reserved or manufacturer-specific code, retained without inventing a meaning.
    #[serde(untagged)]
    Unknown(u8),
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
        _ => Unknown(value),
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_calibration_purposes_retain_their_numeric_value() {
        for raw in 7..=u8::MAX {
            let purpose = CalibrationPurpose::from(raw);
            assert_eq!(purpose, CalibrationPurpose::Unknown(raw));
            assert_eq!(u8::from(purpose), raw);
            assert_eq!(serde_json::to_string(&purpose).unwrap(), raw.to_string());
        }
        assert_eq!(serde_json::to_string(&CalibrationPurpose::from(4)).unwrap(), "\"PeriodicInspection\"");
    }

    #[test]
    fn every_calibration_purpose_roundtrips_to_its_original_byte() {
        for raw in u8::MIN..=u8::MAX {
            assert_eq!(u8::from(CalibrationPurpose::from(raw)), raw);
        }
    }
}
