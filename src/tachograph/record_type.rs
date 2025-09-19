use serde::Serialize;

/// Represents the identifier of a data type.
#[derive(Debug, Clone, Serialize)]
pub enum RecordType {
    /// Unknown data type.
    Unknown = 0,
    /// Information about activity changes.
    ActivityChangeInfo = 1,
    /// Status of the card slots.
    CardSlotStatus = 2,
    /// Current date and time.
    CurrentDateTime,
    /// Member state certificate.
    MemberStateCertificate,
    /// Odometer value at midnight.
    OdometerValueMidnight,
    /// Date of the day the data was downloaded.
    DateOfDayDownloaded,
    /// Information about a paired sensor.
    SensorPaired,
    /// Signature of the data.
    Signature,
    /// Record of specific conditions.
    SpecificConditionRecord,
    /// Vehicle Identification Number (VIN).
    VehicleIdentificationNumber,
    /// Vehicle Registration Number (VRN).
    VehicleRegistrationNumber,
    /// Record of VU calibrations.
    VuCalibrationRecord,
    /// Record of VU card insertion and withdrawal.
    VuCardIWRecord,
    /// Record of VU card data.
    VuCardRecord,
    /// VU certificate.
    VuCertificate,
    /// Record of VU company locks.
    VuCompanyLocksRecord,
    /// Record of VU control activities.
    VuControlActivityRecord,
    /// Block of detailed speed data from the VU.
    VuDetailedSpeedBlock,
    /// Period of downloadable data from the VU.
    VuDownloadablePeriod,
    /// Data about download activities from the VU.
    VuDownloadActivityData,
    /// Record of events from the VU.
    VuEventRecord,
    /// Record of GNSS accumulated driving from the VU.
    VuGNSSADRecord,
    /// Record of ITS consent from the VU.
    VuITSConsentRecord,
    /// Record of faults from the VU.
    VuFaultRecord,
    /// Identification of the VU.
    VuIdentification,
    /// Data about overspeeding control from the VU.
    VuOverSpeedingControlData,
    /// Record of overspeeding events from the VU.
    VuOverSpeedingEventRecord,
    /// Record of daily work periods from the VU.
    VuPlaceDailyWorkPeriodRecord,
    /// Record of time adjustments from GNSS from the VU.
    VuTimeAdjustmentGNSSRecord,
    /// Record of time adjustments from the VU.
    VuTimeAdjustmentRecord,
    /// Record of power supply interruptions from the VU.
    VuPowerSupplyInterruptionRecord,
    /// Record of paired sensors.
    SensorPairedRecord,
    /// Record of external GNSS coupled sensors.
    SensorExternalGNSSCoupledRecord,
    /// Record of border crossings from the VU.
    VuBorderCrossingRecord,
    /// Record of load/unload operations from the VU.
    VuLoadUnloadRecord,
    /// Vehicle Registration Identification.
    VehicleRegistrationIdentification,
}

impl RecordType {
    /// Returns `true` if the data type is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(*self, RecordType::Unknown)
    }
}

impl From<u8> for RecordType {
    fn from(value: u8) -> Self {
        match value {
            1 => RecordType::ActivityChangeInfo,
            2 => RecordType::CardSlotStatus,
            3 => RecordType::CurrentDateTime,
            4 => RecordType::MemberStateCertificate,
            5 => RecordType::OdometerValueMidnight,
            6 => RecordType::DateOfDayDownloaded,
            7 => RecordType::SensorPaired,
            8 => RecordType::Signature,
            9 => RecordType::SpecificConditionRecord,
            10 => RecordType::VehicleIdentificationNumber,
            11 => RecordType::VehicleRegistrationNumber,
            12 => RecordType::VuCalibrationRecord,
            13 => RecordType::VuCardIWRecord,
            14 => RecordType::VuCardRecord,
            15 => RecordType::VuCertificate,
            16 => RecordType::VuCompanyLocksRecord,
            17 => RecordType::VuControlActivityRecord,
            18 => RecordType::VuDetailedSpeedBlock,
            19 => RecordType::VuDownloadablePeriod,
            20 => RecordType::VuDownloadActivityData,
            21 => RecordType::VuEventRecord,
            22 => RecordType::VuGNSSADRecord,
            23 => RecordType::VuITSConsentRecord,
            24 => RecordType::VuFaultRecord,
            25 => RecordType::VuIdentification,
            26 => RecordType::VuOverSpeedingControlData,
            27 => RecordType::VuOverSpeedingEventRecord,
            28 => RecordType::VuPlaceDailyWorkPeriodRecord,
            29 => RecordType::VuTimeAdjustmentGNSSRecord,
            30 => RecordType::VuTimeAdjustmentRecord,
            31 => RecordType::VuPowerSupplyInterruptionRecord,
            32 => RecordType::SensorPairedRecord,
            33 => RecordType::SensorExternalGNSSCoupledRecord,
            34 => RecordType::VuBorderCrossingRecord,
            35 => RecordType::VuLoadUnloadRecord,
            36 => RecordType::VehicleRegistrationIdentification,
            _ => RecordType::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_data_type_id() {
        let data_type_id = RecordType::ActivityChangeInfo;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""ActivityChangeInfo""#);

        let data_type_id = RecordType::VuCalibrationRecord;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""VuCalibrationRecord""#);

        let data_type_id = RecordType::Unknown;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""Unknown""#);
    }
}
