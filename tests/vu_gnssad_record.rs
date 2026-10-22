use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::{VUActivity, VuGnssadRecord};
use esm_parser::tacho::{RecordType, TimeReal, VUTransferResponseParameterID};
use esm_parser::{Export, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn gnss_record(v2: bool, marker: u8, status: u8) -> Vec<u8> {
    let mut bytes = (1_700_000_000 + u32::from(marker)).to_be_bytes().to_vec();
    bytes.extend([1, 13]);
    bytes.extend(format!("TESTDRIVER{marker:06}").as_bytes());
    bytes.push(2);
    bytes.extend([0xFF; 19]);
    bytes.extend((1_699_999_900 + u32::from(marker)).to_be_bytes());
    bytes.push(marker);
    bytes.extend(if marker == 1 { [0, 0xB9, 0x94, 0, 0x0D, 0x06] } else { [0, 0x0D, 0x06, 0, 0xB9, 0x94] });
    if v2 {
        bytes.push(status);
    }
    let odometer = if marker == 1 { 123_456_u32 } else { 654_321 };
    bytes.extend_from_slice(&odometer.to_be_bytes()[1..]);
    assert_eq!(bytes.len(), if v2 { 57 } else { 56 });
    bytes
}

fn array(bytes: &mut Vec<u8>, kind: RecordType, size: u16, records: &[Vec<u8>]) {
    bytes.push(kind as u8);
    bytes.extend(size.to_be_bytes());
    bytes.extend(u16::try_from(records.len()).unwrap().to_be_bytes());
    for record in records {
        assert_eq!(record.len(), usize::from(size));
        bytes.extend(record);
    }
}

fn activity(v2: bool, status: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (kind, size) in [
        (RecordType::DateOfDayDownloaded, 4),
        (RecordType::OdometerValueMidnight, 3),
        (RecordType::VuCardIWRecord, 131),
        (RecordType::ActivityChangeInfo, 2),
        (RecordType::VuPlaceDailyWorkPeriodRecord, if v2 { 41 } else { 40 }),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    array(&mut bytes, RecordType::VuGNSSADRecord, if v2 { 57 } else { 56 }, &[gnss_record(v2, 1, status), gnss_record(v2, 2, 1)]);
    let mut condition = 1_700_000_300_u32.to_be_bytes().to_vec();
    condition.push(1);
    array(&mut bytes, RecordType::SpecificConditionRecord, 5, &[condition]);
    if v2 {
        // F09's border/load arrays remain empty here to isolate GNSS record decoding.
        array(&mut bytes, RecordType::VuBorderCrossingRecord, 55, &[]);
        array(&mut bytes, RecordType::VuLoadUnloadRecord, 58, &[]);
    }
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool, status: u8) -> Vec<u8> {
    // A partial overview supplies the header; this fixture tests activity layout and raw retention.
    let mut bytes = vec![0x76, if v2 { 0x31 } else { 0x21 }];
    for (kind, size) in [
        (RecordType::MemberStateCertificate, 205),
        (RecordType::VuCertificate, 205),
        (RecordType::VehicleIdentificationNumber, 17),
        (
            if v2 { RecordType::VehicleRegistrationIdentification } else { RecordType::VehicleRegistrationNumber },
            if v2 { 15 } else { 14 },
        ),
        (RecordType::CurrentDateTime, 4),
        (RecordType::VuDownloadablePeriod, 8),
        (RecordType::CardSlotStatus, 1),
        (RecordType::VuDownloadActivityData, 59),
        (RecordType::VuCompanyLocksRecord, 99),
        (RecordType::VuControlActivityRecord, 32),
        (RecordType::Signature, 64),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    bytes.extend([0x76, if v2 { 0x32 } else { 0x22 }]);
    bytes.extend(activity(v2, status));
    bytes
}

fn assert_gnss(value: &Value, v2: bool, marker: u8, status: u8) {
    assert_eq!(value["isGen2V2"], v2);
    assert_eq!(value["vehicleOdometerValue"], if marker == 1 { 123_456 } else { 654_321 });
    assert_eq!(value["timeStamp"], json!(TimeReal::new(1_700_000_000 + u32::from(marker))));
    let driver = &value["cardNumberAndGenDriverSlot"];
    assert_eq!(driver["generation"], 2);
    assert_eq!(driver["fullcardNumber"]["cardType"], "DriverCard");
    assert_eq!(driver["fullcardNumber"]["cardIssuingMemberState"], "Germany");
    assert_eq!(driver["fullcardNumber"]["cardNumber"], format!("TESTDRIVER{marker:06}"));
    assert_eq!(value["cardNumberAndGenCodriverSlot"]["fullcardNumber"]["cardType"], "NullCard");
    let place = if v2 {
        assert!(value["gnssPlaceRecord"].is_null());
        assert_eq!(value["gnssPlaceAuthRecord"]["authenticationStatus"], status);
        &value["gnssPlaceAuthRecord"]
    } else {
        assert!(value["gnssPlaceAuthRecord"].is_null());
        &value["gnssPlaceRecord"]
    };
    assert_eq!(place["timeStamp"], json!(TimeReal::new(1_699_999_900 + u32::from(marker))));
    assert_eq!(place["gnssAccuracy"], marker);
    let (latitude, longitude) =
        if marker == 1 { (47.846_666_666_666_67, 3.556_666_666_666_667) } else { (3.556_666_666_666_667, 47.846_666_666_666_67) };
    assert!((place["geoCoordinates"]["latitude"].as_f64().unwrap() - latitude).abs() < 1e-9);
    assert!((place["geoCoordinates"]["longitude"].as_f64().unwrap() - longitude).abs() < 1e-9);
}

#[test]
fn v1_and_v2_gnss_records_preserve_status_odometer_and_consecutive_boundaries() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2Activities), (true, VUTransferResponseParameterID::Gen2v2Activities)]
    {
        for status in [0, 1, 0xFE] {
            let mut bytes = vec![0xCC; 3];
            bytes.extend(gnss_record(v2, 1, status));
            bytes.extend(gnss_record(v2, 2, 1));
            bytes.extend([0xAA, 0xBB]);
            let mut reader = BinMemoryBuffer::from(bytes);
            reader.seek(3).unwrap();
            for marker in 1..=2 {
                let record = VuGnssadRecord::read(&mut reader, &trep).unwrap();
                assert_gnss(&serde_json::to_value(record).unwrap(), v2, marker, if marker == 1 { status } else { 1 });
                assert_eq!(reader.pos().unwrap(), 3 + usize::from(marker) * if v2 { 57 } else { 56 });
            }
            assert_eq!(reader.read_bytes::<2>().unwrap(), [0xAA, 0xBB]);
            assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
        }
    }
}

#[test]
fn gnss_records_reject_every_truncated_length() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2Activities), (true, VUTransferResponseParameterID::Gen2v2Activities)]
    {
        let bytes = gnss_record(v2, 1, 1);
        for length in 0..bytes.len() {
            assert!(
                VuGnssadRecord::read(&mut BinMemoryBuffer::from(&bytes[..length]), &trep).is_err(),
                "{trep:?}, length {length}"
            );
        }
    }
}

#[test]
fn activity_gnss_arrays_reject_incorrect_declared_record_sizes() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2Activities), (true, VUTransferResponseParameterID::Gen2v2Activities)]
    {
        let size = if v2 { 57_u16 } else { 56 };
        for wrong in [0, size - 1, size + 1] {
            let mut bytes = activity(v2, 1);
            bytes[26..28].copy_from_slice(&wrong.to_be_bytes());
            bytes[28..30].copy_from_slice(&1_u16.to_be_bytes());
            let mut record = gnss_record(v2, 1, 1);
            record.resize(usize::from(wrong), 0);
            bytes.splice(30..30 + usize::from(size) * 2, record);
            assert!(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{trep:?}, size {wrong}");
        }
    }
}

#[test]
fn public_activity_import_preserves_authentication_odometer_and_raw_signed_bytes() {
    for v2 in [false, true] {
        for status in [0, 1, 0xFE] {
            let bytes = activity(v2, status);
            let trep =
                if v2 { VUTransferResponseParameterID::Gen2v2Activities } else { VUTransferResponseParameterID::Gen2Activities };
            let mut reader = BinMemoryBuffer::from(bytes.clone());
            VUActivity::from_data(trep, &mut reader).unwrap();
            assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
            let parsed = parse_from_memory(&download(v2, status)).unwrap();
            let value = serde_json::to_value(&parsed).unwrap();
            let activity = &value["transferResParams"][1]["data"]["Activity"];
            let array = &activity["vuGnssadRecordArray"];
            assert_eq!(array["recordSize"], if v2 { 57 } else { 56 });
            assert_eq!(array["noOfRecords"], 2);
            for marker in 1..=2 {
                assert_gnss(&array["records"][usize::from(marker - 1)], v2, marker, if marker == 1 { status } else { 1 });
            }
            let condition = &activity["vuSpecificConditionRecordArray"]["records"][0];
            assert_eq!(condition["entryTime"], json!(TimeReal::new(1_700_000_300)));
            assert_eq!(condition["specificConditionType"], "OutOfScopeBegin");
            let raw = &value["dataFiles"][1];
            assert_eq!(raw["rawData"], json!(bytes));
            assert_eq!(raw["data"], json!(&bytes[..bytes.len() - 69]));
            assert_eq!(raw["signature"], json!(vec![0x55; 64]));
            let xml = parsed.to_xml().unwrap();
            if v2 {
                assert!(xml.contains("<gnssPlaceAuthRecord>"));
                assert!(xml.contains(&format!("<authenticationStatus>{status}</authenticationStatus>")));
            }
            assert!(xml.contains("<vehicleOdometerValue>123456</vehicleOdometerValue>"));
            assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
        }
    }
}
