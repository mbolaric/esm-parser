use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::VuCardRecord;
use esm_parser::tacho::{EquipmentType, RecordType};
use esm_parser::{Export, Readable, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn record(kind: u8, generation: u8) -> Vec<u8> {
    let mut bytes = vec![kind, 13];
    bytes.extend(b"ZYXWVUTSRQPON987");
    bytes.push(generation);
    bytes.extend(1234_u32.to_be_bytes());
    bytes.extend([0x10, 0x26, kind, 1]);
    bytes.extend([1, 2]);
    bytes.extend(b"ABCDEFGHIJKLM123");
    assert_eq!(bytes.len(), 45);
    bytes
}

fn assert_number(value: &Value, kind: u8) {
    let (identification, consecutive, replacement, renewal) = match kind {
        1 => ("ABCDEFGHIJKLM1", "", "2", "3"),
        2..=4 => ("ABCDEFGHIJKLM", "1", "2", "3"),
        _ => ("", "", "", ""),
    };
    assert_eq!(value["number"], "ABCDEFGHIJKLM123");
    assert_eq!(value["equipmentType"], json!(EquipmentType::from(kind)));
    assert_eq!(value["identification"], identification);
    assert_eq!(value["cardConsecutiveindex"], consecutive);
    assert_eq!(value["cardReplacementindex"], replacement);
    assert_eq!(value["cardRenewalindex"], renewal);
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

fn technical_data(v2: bool, records: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (kind, size) in [
        (RecordType::VuIdentification, if v2 { 138 } else { 126 }),
        (RecordType::SensorPairedRecord, 28),
        (RecordType::SensorExternalGNSSCoupledRecord, 28),
        (RecordType::VuCalibrationRecord, if v2 { 252 } else { 222 }),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    array(&mut bytes, RecordType::VuCardRecord, 45, records);
    array(&mut bytes, RecordType::VuITSConsentRecord, 20, &[]);
    array(&mut bytes, RecordType::VuPowerSupplyInterruptionRecord, 87, &[]);
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool, records: &[Vec<u8>]) -> Vec<u8> {
    // A partial overview and dummy signature isolate card-record dispatch and raw retention.
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
    bytes.extend(technical_data(v2, records));
    bytes
}

#[test]
fn vu_card_numbers_use_the_recorded_type_and_preserve_both_number_fields() {
    for generation in [1, 2] {
        for kind in 1..=4 {
            let mut bytes = vec![0xCC; 3];
            bytes.extend(record(kind, generation));
            bytes.push(0xAA);
            let mut reader = BinMemoryBuffer::from(bytes);
            reader.seek(3).unwrap();
            let parsed = VuCardRecord::read(&mut reader).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            let full = &value["cardNumberAndGenerationInformation"];
            assert_eq!(full["fullcardNumber"]["cardType"], json!(EquipmentType::from(kind)));
            assert_eq!(full["fullcardNumber"]["cardNumber"], "ZYXWVUTSRQPON987");
            assert_eq!(full["generation"], generation);
            assert_number(&value["cardNumber"], kind);
            assert_eq!(value["cardExtendedSerialNumber"]["serialNumber"], 1234);
            assert_eq!(value["cardStructureVersion"], json!({"structureVersion":1,"dataElementUseVersion":2}));
            assert_eq!(reader.pos().unwrap(), 48);
            assert_eq!(reader.read_u8().unwrap(), 0xAA);
        }
    }
}

#[test]
fn unsupported_or_null_card_types_retain_the_number_without_inventing_its_parts() {
    for kind in 0..=255 {
        if (1..=4).contains(&kind) {
            continue;
        }
        let mut reader = BinMemoryBuffer::from(record(kind, 2));
        let parsed = VuCardRecord::read(&mut reader).unwrap();
        assert_eq!(parsed.card_number.equipment_type, EquipmentType::from(kind));
        assert_number(&serde_json::to_value(&parsed.card_number).unwrap(), kind);
        assert_eq!(reader.pos().unwrap(), 45);
        #[cfg(feature = "write")]
        {
            use esm_parser::writer::Writable;

            let mut writer = BinMemoryBuffer::new();
            parsed.card_number.write(&mut writer).unwrap();
            assert_eq!(writer.as_slice(), b"ABCDEFGHIJKLM123");
        }
    }
}

#[test]
fn vu_card_records_reject_truncation_and_invalid_card_number_encoding() {
    for kind in [1, 2, 3, 4, 0x80, 255] {
        let bytes = record(kind, 2);
        for length in 0..45 {
            assert!(VuCardRecord::read(&mut BinMemoryBuffer::from(&bytes[..length])).is_err());
        }
        for index in [29, 36, 44] {
            let mut invalid = bytes.clone();
            invalid[index] = 0xFF;
            assert!(VuCardRecord::read(&mut BinMemoryBuffer::from(invalid)).is_err());
        }
    }
}

#[test]
fn v1_and_v2_downloads_preserve_card_types_number_parts_raw_records_and_signatures() {
    let kinds = [1, 2, 3, 4, 0, 0x80, 254, 255];
    let records: Vec<_> = kinds.into_iter().map(|kind| record(kind, 2)).collect();
    for v2 in [false, true] {
        let parsed = parse_from_memory(&download(v2, &records)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let array = &value["transferResParams"][1]["data"]["Calibration"]["vuCardRecordArray"];
        assert_eq!(array["recordSize"], 45);
        assert_eq!(array["noOfRecords"], kinds.len());
        for (record, kind) in array["records"].as_array().unwrap().iter().zip(kinds) {
            assert_number(&record["cardNumber"], kind);
        }
        let raw = technical_data(v2, &records);
        assert_eq!(value["dataFiles"][1]["rawData"], json!(raw));
        assert_eq!(value["dataFiles"][1]["data"], json!(&raw[..raw.len() - 69]));
        assert_eq!(value["dataFiles"][1]["signature"], json!(vec![0x55; 64]));
        assert!(parsed.to_xml().unwrap().contains("<equipmentType>CompanyCard</equipmentType>"));
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
        let mut invalid = record(2, 2);
        invalid[29] = 0xFF;
        assert!(parse_from_memory(&download(v2, &[invalid])).is_err());
    }
}
