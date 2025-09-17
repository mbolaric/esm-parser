use serde::Serialize;

/// Represents the identifier of a data type.
#[derive(Debug, Clone, Serialize)]
pub enum DataTypeID {
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

impl DataTypeID {
    /// Returns `true` if the data type is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(*self, DataTypeID::Unknown)
    }
}

impl From<u8> for DataTypeID {
    fn from(value: u8) -> Self {
        match value {
            1 => DataTypeID::ActivityChangeInfo,
            2 => DataTypeID::CardSlotStatus,
            3 => DataTypeID::CurrentDateTime,
            4 => DataTypeID::MemberStateCertificate,
            5 => DataTypeID::OdometerValueMidnight,
            6 => DataTypeID::DateOfDayDownloaded,
            7 => DataTypeID::SensorPaired,
            8 => DataTypeID::Signature,
            9 => DataTypeID::SpecificConditionRecord,
            10 => DataTypeID::VehicleIdentificationNumber,
            11 => DataTypeID::VehicleRegistrationNumber,
            12 => DataTypeID::VuCalibrationRecord,
            13 => DataTypeID::VuCardIWRecord,
            14 => DataTypeID::VuCardRecord,
            15 => DataTypeID::VuCertificate,
            16 => DataTypeID::VuCompanyLocksRecord,
            17 => DataTypeID::VuControlActivityRecord,
            18 => DataTypeID::VuDetailedSpeedBlock,
            19 => DataTypeID::VuDownloadablePeriod,
            20 => DataTypeID::VuDownloadActivityData,
            21 => DataTypeID::VuEventRecord,
            22 => DataTypeID::VuGNSSADRecord,
            23 => DataTypeID::VuITSConsentRecord,
            24 => DataTypeID::VuFaultRecord,
            25 => DataTypeID::VuIdentification,
            26 => DataTypeID::VuOverSpeedingControlData,
            27 => DataTypeID::VuOverSpeedingEventRecord,
            28 => DataTypeID::VuPlaceDailyWorkPeriodRecord,
            29 => DataTypeID::VuTimeAdjustmentGNSSRecord,
            30 => DataTypeID::VuTimeAdjustmentRecord,
            31 => DataTypeID::VuPowerSupplyInterruptionRecord,
            32 => DataTypeID::SensorPairedRecord,
            33 => DataTypeID::SensorExternalGNSSCoupledRecord,
            34 => DataTypeID::VuBorderCrossingRecord,
            35 => DataTypeID::VuLoadUnloadRecord,
            36 => DataTypeID::VehicleRegistrationIdentification,
            _ => DataTypeID::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_data_type_id() {
        let data_type_id = DataTypeID::ActivityChangeInfo;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""ActivityChangeInfo""#);

        let data_type_id = DataTypeID::VuCalibrationRecord;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""VuCalibrationRecord""#);

        let data_type_id = DataTypeID::Unknown;
        let serialized = serde_json::to_string(&data_type_id).unwrap();
        assert_eq!(serialized, r#""Unknown""#);
    }
}
