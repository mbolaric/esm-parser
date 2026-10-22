use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::{VUActivity, VuBorderCrossingRecord, VuLoadUnloadRecord};
use esm_parser::tacho::{RecordType, TimeReal, VUTransferResponseParameterID};
use esm_parser::{Export, Readable, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn cards(marker: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (slot, kind, country, generation) in [(0, 1, 13, 1), (1, 2, 17, 2)] {
        bytes.extend([kind, country]);
        bytes.extend(format!("TEST{marker:010}{slot:02}").as_bytes());
        bytes.push(generation);
    }
    assert_eq!(bytes.len(), 38);
    bytes
}

fn gnss(marker: u8) -> Vec<u8> {
    let mut bytes = (1_700_000_000 + u32::from(marker)).to_be_bytes().to_vec();
    bytes.push(marker);
    bytes.extend(if marker == 2 { [0, 0x0D, 6, 0, 0xB9, 0x94] } else { [0, 0xB9, 0x94, 0, 0x0D, 6] });
    bytes.push(match marker {
        1 => 0,
        2 => 1,
        _ => 0xFE,
    });
    bytes
}

fn odometer(marker: u8) -> [u8; 3] {
    let value = match marker {
        1 => 123_456_u32,
        2 => 654_321,
        _ => 42,
    };
    value.to_be_bytes()[1..].try_into().unwrap()
}

fn border(marker: u8) -> Vec<u8> {
    let mut bytes = cards(marker);
    bytes.extend(if marker == 1 { [13, 17] } else { [17, 0xFF] });
    bytes.extend(gnss(marker));
    bytes.extend(odometer(marker));
    assert_eq!(bytes.len(), 55);
    bytes
}

fn load(marker: u8) -> Vec<u8> {
    let mut bytes = (1_700_000_200 + u32::from(marker)).to_be_bytes().to_vec();
    bytes.push(marker);
    bytes.extend(cards(marker));
    bytes.extend(gnss(marker));
    bytes.extend(odometer(marker));
    assert_eq!(bytes.len(), 58);
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

fn array_offset(bytes: &[u8], kind: RecordType) -> usize {
    let mut offset = 0;
    while offset + 5 <= bytes.len() {
        if bytes[offset] == kind as u8 {
            return offset;
        }
        let size = usize::from(u16::from_be_bytes(bytes[offset + 1..offset + 3].try_into().unwrap()));
        let count = usize::from(u16::from_be_bytes(bytes[offset + 3..offset + 5].try_into().unwrap()));
        offset += 5 + size * count;
    }
    panic!("missing {kind:?}");
}

fn activity(v2: bool, populated: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (kind, size) in [
        (RecordType::DateOfDayDownloaded, 4),
        (RecordType::OdometerValueMidnight, 3),
        (RecordType::VuCardIWRecord, 131),
        (RecordType::ActivityChangeInfo, 2),
        (RecordType::VuPlaceDailyWorkPeriodRecord, if v2 { 41 } else { 40 }),
        (RecordType::VuGNSSADRecord, if v2 { 57 } else { 56 }),
        (RecordType::SpecificConditionRecord, 5),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    if v2 {
        array(&mut bytes, RecordType::VuBorderCrossingRecord, 55, &if populated { vec![border(1), border(2)] } else { vec![] });
        array(&mut bytes, RecordType::VuLoadUnloadRecord, 58, &if populated { vec![load(1), load(2), load(3)] } else { vec![] });
    }
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool, populated: bool) -> Vec<u8> {
    // A partial overview supplies the header; these byte fixtures test activity layout, not certification.
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
    bytes.extend(activity(v2, populated));
    bytes
}

fn assert_common(value: &Value, marker: u8) {
    for (slot, field, kind, country, generation) in [
        (0, "cardNumberAndGenDriverSlot", "DriverCard", "Germany", 1),
        (1, "cardNumberAndGenCodriverSlot", "WorkshopCard", "France", 2),
    ] {
        let card = &value[field];
        assert_eq!(card["generation"], generation);
        assert_eq!(card["fullcardNumber"]["cardType"], kind);
        assert_eq!(card["fullcardNumber"]["cardIssuingMemberState"], country);
        assert_eq!(card["fullcardNumber"]["cardNumber"], format!("TEST{marker:010}{slot:02}"));
    }
    let place = &value["gnssPlaceAuthRecord"];
    assert_eq!(place["timeStamp"], json!(TimeReal::new(1_700_000_000 + u32::from(marker))));
    assert_eq!(place["gnssAccuracy"], marker);
    assert_eq!(
        place["authenticationStatus"],
        match marker {
            1 => 0,
            2 => 1,
            _ => 0xFE,
        }
    );
    let (latitude, longitude) =
        if marker == 2 { (3.556_666_666_666_667, 47.846_666_666_666_67) } else { (47.846_666_666_666_67, 3.556_666_666_666_667) };
    assert!((place["geoCoordinates"]["latitude"].as_f64().unwrap() - latitude).abs() < 1e-9);
    assert!((place["geoCoordinates"]["longitude"].as_f64().unwrap() - longitude).abs() < 1e-9);
    assert_eq!(
        value["vehicleOdometerValue"],
        match marker {
            1 => 123456,
            2 => 654321,
            _ => 42,
        }
    );
}

fn assert_border(value: &Value, marker: u8) {
    assert_common(value, marker);
    assert_eq!(value["countryLeft"], if marker == 1 { "Germany" } else { "France" });
    assert_eq!(value["countryEntered"], if marker == 1 { "France" } else { "RestOfTheWorld" });
    assert!(value.get("timeStamp").is_none());
}

fn assert_load(value: &Value, marker: u8) {
    assert_common(value, marker);
    assert_eq!(value["timeStamp"], json!(TimeReal::new(1_700_000_200 + u32::from(marker))));
    assert_eq!(
        value["operationType"],
        match marker {
            1 => "Load",
            2 => "Unload",
            _ => "SimultaneousLoadAndUnload",
        }
    );
    assert!(value.get("country").is_none());
    assert!(value.get("region").is_none());
}

#[test]
fn border_reader_consumes_two_distinct_records_and_rejects_every_truncated_length() {
    let mut bytes = vec![0xCC; 3];
    bytes.extend(border(1));
    bytes.extend(border(2));
    bytes.push(0xAA);
    let mut reader = BinMemoryBuffer::from(bytes);
    reader.seek(3).unwrap();
    for marker in 1..=2 {
        let value = serde_json::to_value(VuBorderCrossingRecord::read(&mut reader).unwrap()).unwrap();
        assert_border(&value, marker);
        assert_eq!(value.as_object().unwrap().len(), 6);
        assert_eq!(reader.pos().unwrap(), 3 + usize::from(marker) * 55);
    }
    assert_eq!(reader.read_u8().unwrap(), 0xAA);
    assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    let bytes = border(1);
    for length in 0..55 {
        assert!(VuBorderCrossingRecord::read(&mut BinMemoryBuffer::from(&bytes[..length])).is_err(), "length {length}");
    }
}

#[test]
fn load_reader_consumes_all_three_operations_and_rejects_every_truncated_length() {
    let mut bytes = vec![0xCC; 3];
    for marker in 1..=3 {
        bytes.extend(load(marker));
    }
    bytes.push(0xAA);
    let mut reader = BinMemoryBuffer::from(bytes);
    reader.seek(3).unwrap();
    for marker in 1..=3 {
        let value = serde_json::to_value(VuLoadUnloadRecord::read(&mut reader).unwrap()).unwrap();
        assert_load(&value, marker);
        assert_eq!(value.as_object().unwrap().len(), 6);
        assert_eq!(reader.pos().unwrap(), 3 + usize::from(marker) * 58);
    }
    assert_eq!(reader.read_u8().unwrap(), 0xAA);
    assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    let bytes = load(1);
    for length in 0..58 {
        assert!(VuLoadUnloadRecord::read(&mut BinMemoryBuffer::from(&bytes[..length])).is_err(), "length {length}");
    }
}

#[test]
fn unavailable_cards_and_positions_do_not_remove_vu_records_or_shift_following_arrays() {
    let mut bytes = activity(true, true);
    let border_start = array_offset(&bytes, RecordType::VuBorderCrossingRecord) + 5;
    bytes[border_start..border_start + 40].fill(0xFF);
    let load_start = array_offset(&bytes, RecordType::VuLoadUnloadRecord) + 5;
    bytes[load_start..load_start + 4].fill(0xFF);
    bytes[load_start + 5..load_start + 43].fill(0xFF);
    for start in [border_start + 40, load_start + 43] {
        bytes[start..start + 5].fill(0xFF);
        bytes[start + 5..start + 11].copy_from_slice(&[0x7F, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF]);
        bytes[start + 11] = 0xFE;
    }
    let value = serde_json::to_value(
        VUActivity::from_data(VUTransferResponseParameterID::Gen2v2Activities, &mut BinMemoryBuffer::from(bytes)).unwrap(),
    )
    .unwrap();
    for (field, count) in [("vuBorderCrossingRecordArray", 2), ("vuLoadUnloadRecordArray", 3)] {
        let records = value[field]["records"].as_array().unwrap();
        assert_eq!(records.len(), count);
        let record = &records[0];
        for slot in ["cardNumberAndGenDriverSlot", "cardNumberAndGenCodriverSlot"] {
            assert_eq!(record[slot]["generation"], 0xFF);
            assert_eq!(record[slot]["fullcardNumber"]["cardType"], "NullCard");
            assert_eq!(record[slot]["fullcardNumber"]["cardNumber"], "");
        }
        let gnss = &record["gnssPlaceAuthRecord"];
        assert!(gnss["timeStamp"].is_null());
        assert!(gnss["geoCoordinates"]["latitude"].is_null());
        assert!(gnss["geoCoordinates"]["longitude"].is_null());
        assert_eq!(gnss["authenticationStatus"], 0xFE);
        assert_eq!(record["vehicleOdometerValue"], 123456);
        if count == 2 {
            assert_eq!(record["countryLeft"], "RestOfTheWorld");
            assert_eq!(record["countryEntered"], "RestOfTheWorld");
            assert_border(&records[1], 2);
        } else {
            assert!(record["timeStamp"].is_null());
            assert_eq!(record["operationType"], "Load");
            assert_load(&records[1], 2);
            assert_load(&records[2], 3);
        }
    }
}

#[test]
fn public_activity_import_exports_border_and_load_arrays_with_raw_signed_bytes() {
    for (v2, populated) in [(false, false), (true, false), (true, true)] {
        let bytes = activity(v2, populated);
        let trep =
            if v2 { VUTransferResponseParameterID::Gen2v2Activities } else { VUTransferResponseParameterID::Gen2Activities };
        let mut reader = BinMemoryBuffer::from(bytes.clone());
        VUActivity::from_data(trep, &mut reader).unwrap();
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
        let parsed = parse_from_memory(&download(v2, populated)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let activity = &value["transferResParams"][1]["data"]["Activity"];
        for (field, size, count, kind) in [
            ("vuBorderCrossingRecordArray", 55, if populated { 2 } else { 0 }, "VuBorderCrossingRecord"),
            ("vuLoadUnloadRecordArray", 58, if populated { 3 } else { 0 }, "VuLoadUnloadRecord"),
        ] {
            if v2 {
                let array = &activity[field];
                assert_eq!(array["recordSize"], size);
                assert_eq!(array["recordType"], kind);
                assert_eq!(array["noOfRecords"], count);
                assert_eq!(array["records"].as_array().unwrap().len(), count as usize);
                for marker in 1..=count as u8 {
                    let record = &array["records"][usize::from(marker - 1)];
                    if size == 55 {
                        assert_border(record, marker);
                    } else {
                        assert_load(record, marker);
                    }
                }
            } else {
                assert!(activity[field].is_null());
            }
        }
        let raw = &value["dataFiles"][1];
        assert_eq!(raw["rawData"], json!(bytes));
        assert_eq!(raw["data"], json!(&bytes[..bytes.len() - 69]));
        assert_eq!(raw["signature"], json!(vec![0x55; 64]));
        let xml = parsed.to_xml().unwrap();
        if populated {
            assert!(xml.contains("<countryEntered>RestOfTheWorld</countryEntered>"));
            assert!(xml.contains("<operationType>SimultaneousLoadAndUnload</operationType>"));
        }
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}

#[test]
fn changing_border_or_operation_bytes_changes_only_the_corresponding_decoded_array() {
    let bytes = activity(true, true);
    let trep = VUTransferResponseParameterID::Gen2v2Activities;
    let before = serde_json::to_value(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(bytes.clone())).unwrap()).unwrap();
    let mut changed = bytes.clone();
    changed[array_offset(&bytes, RecordType::VuBorderCrossingRecord) + 5 + 39] = 1;
    let after = serde_json::to_value(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(changed)).unwrap()).unwrap();
    assert_eq!(after["vuBorderCrossingRecordArray"]["records"][0]["countryEntered"], "Austria");
    assert_ne!(before["vuBorderCrossingRecordArray"], after["vuBorderCrossingRecordArray"]);
    assert_eq!(before["vuLoadUnloadRecordArray"], after["vuLoadUnloadRecordArray"]);
    assert_eq!(before["signatureRecordArray"], after["signatureRecordArray"]);

    let mut changed = bytes.clone();
    changed[array_offset(&bytes, RecordType::VuLoadUnloadRecord) + 5 + 4] = 2;
    let after = serde_json::to_value(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(changed)).unwrap()).unwrap();
    assert_eq!(after["vuLoadUnloadRecordArray"]["records"][0]["operationType"], "Unload");
    assert_ne!(before["vuLoadUnloadRecordArray"], after["vuLoadUnloadRecordArray"]);
    assert_eq!(before["vuBorderCrossingRecordArray"], after["vuBorderCrossingRecordArray"]);
    assert_eq!(before["signatureRecordArray"], after["signatureRecordArray"]);
}

#[test]
fn border_and_load_arrays_reject_bad_types_sizes_missing_sections_and_truncated_payloads() {
    let trep = VUTransferResponseParameterID::Gen2v2Activities;
    for (kind, size, record) in
        [(RecordType::VuBorderCrossingRecord, 55_u16, border(1)), (RecordType::VuLoadUnloadRecord, 58, load(1))]
    {
        for populated in [false, true] {
            let original = activity(true, populated);
            let offset = array_offset(&original, kind);
            for wrong in [0, size - 1, size + 1] {
                let mut bytes = original.clone();
                bytes[offset + 1..offset + 3].copy_from_slice(&wrong.to_be_bytes());
                if populated {
                    let count = usize::from(u16::from_be_bytes(bytes[offset + 3..offset + 5].try_into().unwrap()));
                    let mut record = record.clone();
                    record.resize(usize::from(wrong), 0);
                    bytes.splice(offset + 5..offset + 5 + usize::from(size) * count, record.repeat(count));
                }
                assert!(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{kind:?}, size {wrong}");
            }
            let mut bytes = original.clone();
            bytes[offset] = RecordType::VuGNSSADRecord as u8;
            assert!(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{kind:?}, wrong type");
            let end = offset + 5 + if populated { usize::from(size) * if size == 55 { 2 } else { 3 } } else { 0 };
            let mut bytes = original.clone();
            bytes.drain(offset..end);
            assert!(VUActivity::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{kind:?}, missing section");
            for length in [offset, offset + 4, end - 1] {
                assert!(
                    VUActivity::from_data(trep, &mut BinMemoryBuffer::from(&original[..length])).is_err(),
                    "{kind:?}, length {length}"
                );
            }
        }
    }
}
