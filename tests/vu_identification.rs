use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::{VUTechnicalData, VuIdentification};
use esm_parser::tacho::{RecordType, VUTransferResponseParameterID};
use esm_parser::{Export, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn identification_record(v2: bool, marker: u8, map_version: &[u8; 12]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in [format!("TEST MANUFACTURER {marker}"), format!("TEST ADDRESS {marker}")] {
        bytes.push(1);
        bytes.extend(value.as_bytes());
        bytes.resize(bytes.len() + 35 - value.len(), b' ');
    }
    bytes.extend(format!("TESTPART{marker:08}").as_bytes());
    bytes.extend((1000 + u32::from(marker)).to_be_bytes());
    bytes.extend([0x06, 0x24, 6, 0x10 + marker]);
    bytes.extend(format!("V{marker:03}").as_bytes());
    bytes.extend((1_700_000_000 + u32::from(marker)).to_be_bytes());
    bytes.extend((1_600_000_000 + u32::from(marker)).to_be_bytes());
    bytes.extend(format!("TESTAPPROVAL{marker:04}").as_bytes());
    bytes.extend([2, marker]);
    assert_eq!(bytes.len(), 126);
    if v2 {
        bytes.extend(map_version);
    }
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

fn technical_data(v2: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    array(
        &mut bytes,
        RecordType::VuIdentification,
        if v2 { 138 } else { 126 },
        &[identification_record(v2, 1, b"MAP202608001"), identification_record(v2, 2, b"MAP202609002")],
    );
    for kind in [RecordType::SensorPairedRecord, RecordType::SensorExternalGNSSCoupledRecord] {
        array(&mut bytes, kind, 28, &[]);
    }
    let mut calibration = vec![0; if v2 { 252 } else { 222 }];
    calibration[0] = 4;
    array(&mut bytes, RecordType::VuCalibrationRecord, if v2 { 252 } else { 222 }, &[calibration]);
    for (kind, size) in
        [(RecordType::VuCardRecord, 45), (RecordType::VuITSConsentRecord, 20), (RecordType::VuPowerSupplyInterruptionRecord, 87)]
    {
        array(&mut bytes, kind, size, &[]);
    }
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool) -> Vec<u8> {
    // A partial overview supplies the header; this fixture isolates technical record layout and raw retention.
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
    bytes.extend([0x76, if v2 { 0x35 } else { 0x25 }]);
    bytes.extend(technical_data(v2));
    bytes
}

fn assert_identification(value: &Value, v2: bool, marker: u8) {
    assert_eq!(value["isGen2V2"], v2);
    assert_eq!(value["vuManufacturerName"], format!("TEST MANUFACTURER {marker}"));
    assert_eq!(value["vuManufacturerAddress"], format!("TEST ADDRESS {marker}"));
    assert_eq!(value["vuPartNumber"], format!("TESTPART{marker:08}"));
    assert_eq!(value["vuSerialNumber"]["serialNumber"], 1000 + u32::from(marker));
    assert_eq!(value["vuSerialNumber"]["monthYear"], "0624");
    assert_eq!(value["vuSerialNumber"]["type"], 6);
    assert_eq!(value["vuSerialNumber"]["manufacturerCode"], 0x10 + marker);
    assert_eq!(value["vuSoftwareIdentification"]["vuSoftwareVersion"], format!("V{marker:03}"));
    assert_eq!(value["vuApprovalNumber"], format!("TESTAPPROVAL{marker:04}"));
    assert_eq!(value["vuGeneration"], 2);
    assert_eq!(value["vuAbility"], marker);
    if v2 {
        assert_eq!(value["vuDigitalMapVersion"], if marker == 1 { "MAP202608001" } else { "MAP202609002" });
    } else {
        assert!(value["vuDigitalMapVersion"].is_null());
    }
}

#[test]
fn v1_and_v2_identification_records_keep_distinct_map_versions_and_exact_boundaries() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let mut bytes = vec![0xCC; 3];
        bytes.extend(identification_record(v2, 1, b"MAP202608001"));
        bytes.extend(identification_record(v2, 2, b"MAP202609002"));
        bytes.extend([0xAA, 0xBB]);
        let mut reader = BinMemoryBuffer::from(bytes);
        reader.seek(3).unwrap();
        for marker in 1..=2 {
            let record = VuIdentification::read(&mut reader, &trep).unwrap();
            assert_identification(&serde_json::to_value(record).unwrap(), v2, marker);
            assert_eq!(reader.pos().unwrap(), 3 + usize::from(marker) * if v2 { 138 } else { 126 });
        }
        assert_eq!(reader.read_bytes::<2>().unwrap(), [0xAA, 0xBB]);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }
}

#[test]
fn identification_records_reject_every_truncated_length() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let bytes = identification_record(v2, 1, b"MAP202608001");
        for length in 0..bytes.len() {
            assert!(
                VuIdentification::read(&mut BinMemoryBuffer::from(&bytes[..length]), &trep).is_err(),
                "{trep:?}, length {length}"
            );
        }
    }
}

#[test]
fn map_versions_use_ia5_padding_and_the_unavailable_default() {
    let trep = VUTransferResponseParameterID::Gen2v2TechnicalData;
    for (raw, expected) in
        [(*b"MAP2026     ", json!("MAP2026")), (*b"MAP2026\0\0\0\0\0", json!("MAP2026")), ([0xFF; 12], Value::Null)]
    {
        let mut reader = BinMemoryBuffer::from(identification_record(true, 1, &raw));
        let value = serde_json::to_value(VuIdentification::read(&mut reader, &trep).unwrap()).unwrap();
        assert_eq!(value["vuDigitalMapVersion"], expected);
        assert_eq!(reader.pos().unwrap(), 138);
    }
    for index in 0..12 {
        let mut raw = *b"MAP202608001";
        raw[index] = 0x80;
        assert!(VuIdentification::read(&mut BinMemoryBuffer::from(identification_record(true, 1, &raw)), &trep).is_err());
    }
    let mut raw = [0xFF; 12];
    raw[0] = b'M';
    assert!(VuIdentification::read(&mut BinMemoryBuffer::from(identification_record(true, 1, &raw)), &trep).is_err());
}

#[test]
fn technical_identification_arrays_reject_incorrect_declared_record_sizes() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let size = if v2 { 138_u16 } else { 126 };
        for wrong in [0, size - 1, size + 1] {
            let mut bytes = technical_data(v2);
            bytes[1..3].copy_from_slice(&wrong.to_be_bytes());
            bytes[3..5].copy_from_slice(&1_u16.to_be_bytes());
            let mut record = identification_record(v2, 1, b"MAP202608001");
            record.resize(usize::from(wrong), 0);
            bytes.splice(5..5 + usize::from(size) * 2, record);
            assert!(VUTechnicalData::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{trep:?}, size {wrong}");
        }
    }
}

#[test]
fn public_vu_import_preserves_identification_maps_following_calibration_and_raw_signatures() {
    for v2 in [false, true] {
        let bytes = technical_data(v2);
        let trep = if v2 {
            VUTransferResponseParameterID::Gen2v2TechnicalData
        } else {
            VUTransferResponseParameterID::Gen2TechnicalData
        };
        let mut reader = BinMemoryBuffer::from(bytes.clone());
        VUTechnicalData::from_data(trep, &mut reader).unwrap();
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
        let parsed = parse_from_memory(&download(v2)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let technical = &value["transferResParams"][1]["data"]["Calibration"];
        let array = &technical["vuIdentificationRecordArray"];
        assert_eq!(array["recordSize"], if v2 { 138 } else { 126 });
        assert_eq!(array["noOfRecords"], 2);
        for marker in 1..=2 {
            assert_identification(&array["records"][usize::from(marker - 1)], v2, marker);
        }
        let calibration = &technical["vuCalibrationRecordArray"]["records"][0];
        assert_eq!(calibration["calibrationPurpose"], "PeriodicInspection");
        assert_eq!(calibration["isGen2V2"], v2);
        let raw = &value["dataFiles"][1];
        assert_eq!(raw["rawData"], json!(bytes));
        assert_eq!(raw["data"], json!(&bytes[..bytes.len() - 69]));
        assert_eq!(raw["signature"], json!(vec![0x55; 64]));
        let xml = parsed.to_xml().unwrap();
        if v2 {
            assert!(xml.contains("<vuDigitalMapVersion>MAP202608001</vuDigitalMapVersion>"));
            assert!(xml.contains("<vuDigitalMapVersion>MAP202609002</vuDigitalMapVersion>"));
        }
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}
