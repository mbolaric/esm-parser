use serde::Serialize;

use crate::impl_enum_from_u8;

/// Code qualifying an event or a fault (Annex IC, Appendix 1, 2.70).
/// Includes generation 2 additions from Regulations (EU) 2018/502 and 2021/1228.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum EventFaultType {
    NoFurtherDetails = 0,
    InsertionOfNonValidCard = 1,
    CardConflict = 2,
    TimeOverlap = 3,
    DrivingWithoutAppropriateCard = 4,
    CardInsertionWhileDriving = 5,
    LastCardSessionNotCorrectlyClosed = 6,
    OverSpeeding = 7,
    PowerSupplyInterruption = 8,
    MotionDataError = 9,
    VehicleMotionConflict = 10,
    TimeConflict = 11,
    RCMRemoteCommunicationModuleCommunicationFault = 12,
    GNSSNoGNSSPositionData = 13,
    GNSSExternalGNSSCommunicationFault = 14,
    GNSSAnomaly = 15,
    VuNoFurtherDetails = 16,
    VuMotionSensorAuthenticationFailure = 17,
    VuTachographCardAuthenticationFailure = 18,
    VuUnauthorisedChangeOfMotionSensor = 19,
    VuCardDataInputIntegrityError = 20,
    VuStoredUserDataIntegrityError = 21,
    VuInternalDataTransferError = 22,
    VuUnauthorisedCaseOpening = 23,
    VuHardwareSabotage = 24,
    GNSSTamperDetectionOfGNSS = 25,
    GNSSExternalGNSSFacilityAuthenticationFailure = 26,
    GNSSExternalGNSSFacilityCertificateExpired = 27,
    VuMotionDataAndDriverActivityInconsistency = 28,
    SensorNoFurtherDetails = 32,
    SensorAuthenticationFailure = 33,
    SensorStoredDataIntegrityError = 34,
    SensorInternalDataTransferError = 35,
    SensorUnauthorisedCaseOpening = 36,
    SensorHardwareSabotage = 37,
    RENoFurtherDetails = 48,
    REVUInternalFault = 49,
    REPrinterFault = 50,
    REDisplayFault = 51,
    REDownloadingFault = 52,
    RESensorFault = 53,
    GNSSInternalGNSSReceiverFault = 54,
    GNSSExternalGNSSReceiverFault = 55,
    RCMRemoteCommunicationModuleFault = 56,
    REITSInterfaceFault = 57,
    CardNoFurtherDetails = 64,
    /// Reserved or manufacturer-specific code, retained without inventing a meaning.
    #[serde(untagged)]
    Unknown(u8) = 255,
}

impl_enum_from_u8!(
    EventFaultType {
        NoFurtherDetails = 0,
        InsertionOfNonValidCard = 1,
        CardConflict = 2,
        TimeOverlap = 3,
        DrivingWithoutAppropriateCard = 4,
        CardInsertionWhileDriving = 5,
        LastCardSessionNotCorrectlyClosed = 6,
        OverSpeeding = 7,
        PowerSupplyInterruption = 8,
        MotionDataError = 9,
        VehicleMotionConflict = 10,
        TimeConflict = 11,
        RCMRemoteCommunicationModuleCommunicationFault = 12,
        GNSSNoGNSSPositionData = 13,
        GNSSExternalGNSSCommunicationFault = 14,
        GNSSAnomaly = 15,
        VuNoFurtherDetails = 16,
        VuMotionSensorAuthenticationFailure = 17,
        VuTachographCardAuthenticationFailure = 18,
        VuUnauthorisedChangeOfMotionSensor = 19,
        VuCardDataInputIntegrityError = 20,
        VuStoredUserDataIntegrityError = 21,
        VuInternalDataTransferError = 22,
        VuUnauthorisedCaseOpening = 23,
        VuHardwareSabotage = 24,
        GNSSTamperDetectionOfGNSS = 25,
        GNSSExternalGNSSFacilityAuthenticationFailure = 26,
        GNSSExternalGNSSFacilityCertificateExpired = 27,
        VuMotionDataAndDriverActivityInconsistency = 28,
        SensorNoFurtherDetails = 32,
        SensorAuthenticationFailure = 33,
        SensorStoredDataIntegrityError = 34,
        SensorInternalDataTransferError = 35,
        SensorUnauthorisedCaseOpening = 36,
        SensorHardwareSabotage = 37,
        RENoFurtherDetails = 48,
        REVUInternalFault = 49,
        REPrinterFault = 50,
        REDisplayFault = 51,
        REDownloadingFault = 52,
        RESensorFault = 53,
        GNSSInternalGNSSReceiverFault = 54,
        GNSSExternalGNSSReceiverFault = 55,
        RCMRemoteCommunicationModuleFault = 56,
        REITSInterfaceFault = 57,
        CardNoFurtherDetails = 64,
        _ => Unknown(value),
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_standard_gen2_codes_at_their_assigned_values() {
        for (raw, expected) in [
            (0x0C, EventFaultType::RCMRemoteCommunicationModuleCommunicationFault),
            (0x0D, EventFaultType::GNSSNoGNSSPositionData),
            (0x0E, EventFaultType::GNSSExternalGNSSCommunicationFault),
            (0x0F, EventFaultType::GNSSAnomaly),
            (0x19, EventFaultType::GNSSTamperDetectionOfGNSS),
            (0x1A, EventFaultType::GNSSExternalGNSSFacilityAuthenticationFailure),
            (0x1B, EventFaultType::GNSSExternalGNSSFacilityCertificateExpired),
            (0x1C, EventFaultType::VuMotionDataAndDriverActivityInconsistency),
            (0x36, EventFaultType::GNSSInternalGNSSReceiverFault),
            (0x37, EventFaultType::GNSSExternalGNSSReceiverFault),
            (0x38, EventFaultType::RCMRemoteCommunicationModuleFault),
            (0x39, EventFaultType::REITSInterfaceFault),
        ] {
            assert_eq!(EventFaultType::from(raw), expected);
        }
    }

    #[test]
    fn preserves_reserved_and_manufacturer_specific_codes() {
        for raw in 0x50..=0xFF {
            let code = EventFaultType::from(raw);
            assert_eq!(code, EventFaultType::Unknown(raw));
            assert_eq!(serde_json::to_string(&code).unwrap(), raw.to_string());
        }
        assert_eq!(serde_json::to_string(&EventFaultType::from(0x31)).unwrap(), "\"REVUInternalFault\"");
    }

    #[test]
    fn every_event_fault_code_roundtrips_to_its_original_byte() {
        for raw in u8::MIN..=u8::MAX {
            assert_eq!(u8::from(EventFaultType::from(raw)), raw);
        }
    }
}
