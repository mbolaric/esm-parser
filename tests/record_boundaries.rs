use binary_data::BinMemoryBuffer;
use esm_parser::Readable;
use esm_parser::gen2::{
    DataInfo, DataInfoGenericRecordArray, DataInfoReadable, DriverCardApplicationIdentificationV2,
    MemberStateCertificateRecordArray, SignatureRecordArray, VehicleIdentificationNumberRecordArray,
    VehicleRegistrationIdentificationRecordArray, VehicleRegistrationNumberRecordArray, VuActivityDailyRecordArray,
    VuCertificateRecordArray, VuPlaceDailyWorkPeriodRecordArray,
};
use esm_parser::tacho::{RecordType, TimeReal, VUTransferResponseParameterID};

fn info(kind: RecordType, size: u16, count: u16, data: Vec<u8>) -> DataInfo {
    DataInfo {
        trep_id: VUTransferResponseParameterID::Gen2Overview,
        record_type: kind,
        record_size: size,
        no_of_records: count,
        data,
    }
}

#[test]
fn generic_arrays_reject_wrong_tags_strides_and_unconsumed_payloads() {
    let valid = info(RecordType::CurrentDateTime, 4, 2, [1_u32.to_be_bytes(), 2_u32.to_be_bytes()].concat());
    let records: DataInfoGenericRecordArray<TimeReal> = valid.parse().unwrap();
    assert_eq!(records.records.iter().map(TimeReal::get_data).collect::<Vec<_>>(), [1, 2]);
    for malformed in [
        info(RecordType::CurrentDateTime, 5, 2, vec![0, 0, 0, 1, 0xAA, 0, 0, 0, 2, 0xBB]),
        info(RecordType::OdometerValueMidnight, 4, 2, valid.data.clone()),
        info(RecordType::CurrentDateTime, 4, 2, vec![0; 9]),
        info(RecordType::CurrentDateTime, 0, 0, vec![]),
        info(RecordType::CurrentDateTime, 5, 0, vec![]),
    ] {
        assert!(malformed.parse::<DataInfoGenericRecordArray<TimeReal>>().is_err());
    }
}

fn rejects_bad_layouts<T: DataInfoReadable>(kind: RecordType, size: u16) {
    assert!(info(kind, size, 0, vec![]).parse::<T>().is_ok());
    for (tag, length) in [(RecordType::Unknown, size), (kind, 0), (kind, size - 1), (kind, size + 1)] {
        assert!(info(tag, length, 0, vec![]).parse::<T>().is_err(), "{tag:?}, size {length}");
    }
}

#[test]
fn custom_arrays_check_tags_and_fixed_sizes_even_when_empty() {
    rejects_bad_layouts::<VehicleIdentificationNumberRecordArray>(RecordType::VehicleIdentificationNumber, 17);
    rejects_bad_layouts::<VehicleRegistrationNumberRecordArray>(RecordType::VehicleRegistrationNumber, 14);
    rejects_bad_layouts::<VehicleRegistrationIdentificationRecordArray>(RecordType::VehicleRegistrationIdentification, 15);
    rejects_bad_layouts::<VuActivityDailyRecordArray>(RecordType::ActivityChangeInfo, 2);
    rejects_bad_layouts::<VuPlaceDailyWorkPeriodRecordArray>(RecordType::VuPlaceDailyWorkPeriodRecord, 40);
    for kind in [RecordType::Unknown, RecordType::MemberStateCertificate] {
        assert!(info(kind, 64, 0, vec![]).parse::<SignatureRecordArray>().is_err());
    }
    assert!(info(RecordType::Unknown, 205, 0, vec![]).parse::<MemberStateCertificateRecordArray>().is_err());
    assert!(info(RecordType::Unknown, 205, 0, vec![]).parse::<VuCertificateRecordArray>().is_err());
}

#[test]
fn application_v2_rejects_unsupported_declared_lengths() {
    for length in [0_u16, 2, 7, 9, u16::MAX] {
        let mut bytes = length.to_be_bytes().to_vec();
        bytes.extend([0; 8]);
        assert!(DriverCardApplicationIdentificationV2::read(&mut BinMemoryBuffer::from(bytes)).is_err());
    }
    let mut valid = 8_u16.to_be_bytes().to_vec();
    valid.extend([0; 8]);
    assert!(DriverCardApplicationIdentificationV2::read(&mut BinMemoryBuffer::from(valid)).is_ok());
}

#[test]
fn oversized_and_zero_size_array_headers_are_rejected_before_allocating_the_payload() {
    for header in [[3, 0xFF, 0xFF, 0xFF, 0xFF], [3, 0, 0, 0, 0], [3, 0, 4, 0, 2]] {
        assert!(
            DataInfo::read(&mut BinMemoryBuffer::from(header.to_vec()), VUTransferResponseParameterID::Gen2Overview).is_err()
        );
    }
}

#[test]
fn every_fixed_vu_record_uses_its_type_and_exact_physical_size() {
    use esm_parser::gen2::{
        SensorExternalGNSSCoupledRecord, SensorPairedRecord, VuBorderCrossingRecord, VuCardIWRecord, VuCardRecord,
        VuCompanyLocksRecord, VuControlActivityRecord, VuDownloadActivityData, VuDownloadablePeriod, VuEventRecord,
        VuFaultRecord, VuItsConsentRecord, VuLoadUnloadRecord, VuOverSpeedingEventRecord, VuPowerSupplyInterruptionRecord,
        VuTimeAdjustmentRecord,
    };
    use esm_parser::tacho::{
        CardSlotStatus, OdometerShort, SpecificConditionRecord, VuDetailedSpeedBlock, VuOverSpeedingControlData,
    };

    macro_rules! check {
        ($record:ty, $tag:ident, $size:expr) => {{
            rejects_bad_layouts::<DataInfoGenericRecordArray<$record>>(RecordType::$tag, $size);
            assert!(
                info(RecordType::$tag, $size, 2, vec![0; 2 * $size as usize])
                    .parse::<DataInfoGenericRecordArray<$record>>()
                    .is_ok()
            );
        }};
    }
    check!(TimeReal, CurrentDateTime, 4);
    check!(OdometerShort, OdometerValueMidnight, 3);
    check!(SpecificConditionRecord, SpecificConditionRecord, 5);
    check!(CardSlotStatus, CardSlotStatus, 1);
    check!(VuDetailedSpeedBlock, VuDetailedSpeedBlock, 64);
    check!(VuOverSpeedingControlData, VuOverSpeedingControlData, 9);
    check!(VuDownloadablePeriod, VuDownloadablePeriod, 8);
    check!(VuDownloadActivityData, VuDownloadActivityData, 59);
    check!(VuCompanyLocksRecord, VuCompanyLocksRecord, 99);
    check!(VuControlActivityRecord, VuControlActivityRecord, 32);
    check!(VuCardIWRecord, VuCardIWRecord, 131);
    check!(VuBorderCrossingRecord, VuBorderCrossingRecord, 55);
    check!(VuLoadUnloadRecord, VuLoadUnloadRecord, 58);
    check!(SensorPairedRecord, SensorPairedRecord, 28);
    check!(SensorExternalGNSSCoupledRecord, SensorExternalGNSSCoupledRecord, 28);
    check!(VuCardRecord, VuCardRecord, 45);
    check!(VuItsConsentRecord, VuITSConsentRecord, 20);
    check!(VuPowerSupplyInterruptionRecord, VuPowerSupplyInterruptionRecord, 87);
    check!(VuEventRecord, VuEventRecord, 91);
    check!(VuFaultRecord, VuFaultRecord, 90);
    check!(VuOverSpeedingEventRecord, VuOverSpeedingEventRecord, 32);
    check!(VuTimeAdjustmentRecord, VuTimeAdjustmentRecord, 99);
}

#[test]
fn versioned_vu_record_layouts_are_checked_even_without_records() {
    use esm_parser::gen2::{VuCalibrationRecord, VuGnssadRecord, VuIdentification};

    macro_rules! check {
        ($record:ty, $tag:ident, $trep:ident, $size:expr) => {{
            for size in [$size - 1, $size, $size + 1] {
                let mut value = info(RecordType::$tag, size, 0, vec![]);
                value.trep_id = VUTransferResponseParameterID::$trep;
                assert_eq!(value.parse_with_params::<DataInfoGenericRecordArray<$record>>().is_ok(), size == $size);
                value.record_type = RecordType::Unknown;
                assert!(value.parse_with_params::<DataInfoGenericRecordArray<$record>>().is_err());
            }
        }};
    }
    check!(VuIdentification, VuIdentification, Gen2TechnicalData, 126);
    check!(VuIdentification, VuIdentification, Gen2v2TechnicalData, 138);
    check!(VuCalibrationRecord, VuCalibrationRecord, Gen2TechnicalData, 222);
    check!(VuCalibrationRecord, VuCalibrationRecord, Gen2v2TechnicalData, 252);
    check!(VuGnssadRecord, VuGNSSADRecord, Gen2Activities, 56);
    check!(VuGnssadRecord, VuGNSSADRecord, Gen2v2Activities, 57);
}

#[test]
fn variable_certificate_readers_cannot_consume_the_following_certificate_slot() {
    for kind in [RecordType::MemberStateCertificate, RecordType::VuCertificate] {
        let mut first = vec![0; 205];
        first[204] = 0x11;
        let mut second = first.clone();
        second[204] = 0x22;
        let array = info(kind, 205, 2, [first.clone(), second.clone()].concat());
        macro_rules! check {
            ($array:ty) => {{
                let actual: $array = array.parse().unwrap();
                assert_eq!(actual.records.len(), 2);
                for (record, raw) in actual.records.iter().zip([first.clone(), second.clone()]) {
                    let expected: $array = info(kind, 205, 1, raw).parse().unwrap();
                    assert_eq!(serde_json::to_value(record).unwrap(), serde_json::to_value(&expected.records[0]).unwrap());
                }
            }};
        }
        match kind {
            RecordType::MemberStateCertificate => check!(MemberStateCertificateRecordArray),
            _ => check!(VuCertificateRecordArray),
        }
    }
}
