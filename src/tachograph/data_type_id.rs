#[derive(Debug)]   
pub enum DataTypeID {
    Unknown = 0,
	ActivityChangeInfo = 1,
	CardSlotStatus = 2,
	CurrentDateTime,
	MemberStateCertificate,
	OdometerValueMidnight,
	DateOfDayDownloaded,
	SensorPaired,
	Signature,
	SpecificConditionRecord,
	VehicleIdentificationNumber,
	VehicleRegistrationNumber,
	VuCalibrationRecord,
	VuCardIWRecord,
	VuCardRecord,
	VuCertificate,
	VuCompanyLocksRecord,
	VuControlActivityRecord,
	VuDetailedSpeedBlock,
	VuDownloadablePeriod,
	VuDownloadActivityData,
	VuEventRecord,
	VuGNSSADRecord,
	VuITSConsentRecord,
	VuFaultRecord,
	VuIdentification,
	VuOverSpeedingControlData,
	VuOverSpeedingEventRecord,
	VuPlaceDailyWorkPeriodRecord,
	VuTimeAdjustmentGNSSRecord,
	VuTimeAdjustmentRecord,
	VuPowerSupplyInterruptionRecord,
	SensorPairedRecord,
	SensorExternalGNSSCoupledRecord,
	VuBorderCrossingRecord,
	VuLoadUnloadRecord,
	VehicleRegistrationIdentification
}

impl DataTypeID {
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
            _ => DataTypeID::Unknown
        }
    }
}