use binary_data::{BinMemoryBuffer, BinSeek};
use esm_parser::gen2::VUOverview;
use esm_parser::tacho::{RecordType, TimeReal, VUTransferResponseParameterID};
use esm_parser::{Export, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn array(bytes: &mut Vec<u8>, kind: RecordType, size: u16, records: &[Vec<u8>]) {
    bytes.push(kind as u8);
    bytes.extend(size.to_be_bytes());
    bytes.extend(u16::try_from(records.len()).unwrap().to_be_bytes());
    for record in records {
        assert_eq!(record.len(), usize::from(size));
        bytes.extend(record);
    }
}

fn overview(v2: bool, countries: &[u8]) -> Vec<u8> {
    // Partial arrays and a dummy signature isolate the overview layout, not certification.
    let mut bytes = Vec::new();
    array(&mut bytes, RecordType::MemberStateCertificate, 205, &[]);
    array(&mut bytes, RecordType::VuCertificate, 205, &[]);
    array(&mut bytes, RecordType::VehicleIdentificationNumber, 17, &[b"TESTVIN1234567890".to_vec()]);
    let records = countries
        .iter()
        .map(|country| {
            let mut record = if v2 { vec![*country] } else { vec![] };
            record.push(1);
            record.extend(b"E-TEST-123   ");
            record
        })
        .collect::<Vec<_>>();
    array(
        &mut bytes,
        if v2 { RecordType::VehicleRegistrationIdentification } else { RecordType::VehicleRegistrationNumber },
        if v2 { 15 } else { 14 },
        &records,
    );
    array(&mut bytes, RecordType::CurrentDateTime, 4, &[1_700_000_000_u32.to_be_bytes().to_vec()]);
    for (kind, size) in [
        (RecordType::VuDownloadablePeriod, 8),
        (RecordType::CardSlotStatus, 1),
        (RecordType::VuDownloadActivityData, 59),
        (RecordType::VuCompanyLocksRecord, 99),
        (RecordType::VuControlActivityRecord, 32),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool, countries: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0x76, if v2 { 0x31 } else { 0x21 }];
    bytes.extend(overview(v2, countries));
    bytes
}

fn decoded_overview(bytes: &[u8]) -> Value {
    serde_json::to_value(parse_from_memory(bytes).unwrap()).unwrap()["transferResParams"][0]["data"]["Control"].clone()
}

#[test]
fn v1_and_v2_overviews_expose_their_actual_registration_layout() {
    for v2 in [false, true] {
        for countries in [&[][..], &[13, 15, 0xFF][..]] {
            let bytes = overview(v2, countries);
            let trep =
                if v2 { VUTransferResponseParameterID::Gen2v2Overview } else { VUTransferResponseParameterID::Gen2Overview };
            let mut reader = BinMemoryBuffer::from(bytes);
            let value = serde_json::to_value(VUOverview::from_data(trep, &mut reader).unwrap()).unwrap();
            assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
            let (present, absent, kind, size) = if v2 {
                (
                    "vehicleRegistrationIdentificationRecordArray",
                    "vehicleRegistrationNumberRecordArray",
                    "VehicleRegistrationIdentification",
                    15,
                )
            } else {
                (
                    "vehicleRegistrationNumberRecordArray",
                    "vehicleRegistrationIdentificationRecordArray",
                    "VehicleRegistrationNumber",
                    14,
                )
            };
            let records = if v2 {
                ["Germany", "Spain", "RestOfTheWorld"]
                    .iter()
                    .take(countries.len())
                    .map(|country| json!({"vehicleRegistrationNation":country,"vehicleRegistrationNumber":"E-TEST-123"}))
                    .collect::<Vec<_>>()
            } else {
                vec![json!("E-TEST-123"); countries.len()]
            };
            assert_eq!(
                value[present],
                json!({"recordType":kind,"recordSize":size,"noOfRecords":countries.len(),"records":records})
            );
            assert!(value.get(absent).unwrap().is_null());
            assert_eq!(value["CurrentDateTimeRecordArray"]["records"], json!([TimeReal::new(1_700_000_000)]));
        }
    }
}

#[test]
fn changing_only_the_registration_country_changes_the_decoded_overview() {
    let before = decoded_overview(&download(true, &[13]));
    let mut after = decoded_overview(&download(true, &[15]));
    assert_ne!(before, after);
    assert_eq!(before["vehicleRegistrationIdentificationRecordArray"]["records"][0]["vehicleRegistrationNation"], "Germany");
    assert_eq!(after["vehicleRegistrationIdentificationRecordArray"]["records"][0]["vehicleRegistrationNation"], "Spain");
    after["vehicleRegistrationIdentificationRecordArray"]["records"][0]["vehicleRegistrationNation"] = json!("Germany");
    assert_eq!(before, after);
}

#[test]
fn public_overview_import_preserves_registration_and_signed_raw_bytes_in_all_exports() {
    for v2 in [false, true] {
        let bytes = overview(v2, &[13, 15]);
        let parsed = parse_from_memory(&download(v2, &[13, 15])).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let raw = &value["dataFiles"][0];
        assert_eq!(raw["rawData"], json!(bytes));
        assert_eq!(raw["data"], json!(&bytes[10..bytes.len() - 69]));
        assert_eq!(raw["signature"], json!(vec![0x55; 64]));
        let xml = parsed.to_xml().unwrap();
        if v2 {
            assert!(xml.contains("<vehicleRegistrationNation>Germany</vehicleRegistrationNation>"));
            assert!(xml.contains("<vehicleRegistrationNation>Spain</vehicleRegistrationNation>"));
        }
        assert!(xml.contains("E-TEST-123"));
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}

#[test]
fn overview_registration_arrays_reject_wrong_record_types_and_sizes_even_when_empty() {
    for v2 in [false, true] {
        let size = if v2 { 15_u16 } else { 14 };
        let offset = 32; // Two empty certificate headers, then the VIN header and its 17 bytes.
        for countries in [&[][..], &[13][..]] {
            for wrong_size in [0, size - 1, size + 1] {
                let mut bytes = overview(v2, countries);
                bytes[offset + 1..offset + 3].copy_from_slice(&wrong_size.to_be_bytes());
                if !countries.is_empty() {
                    let mut record = bytes[offset + 5..offset + 5 + usize::from(size)].to_vec();
                    record.resize(usize::from(wrong_size), 0);
                    bytes.splice(offset + 5..offset + 5 + usize::from(size), record);
                }
                let mut file = vec![0x76, if v2 { 0x31 } else { 0x21 }];
                file.extend(bytes);
                assert!(parse_from_memory(&file).is_err(), "v2 {v2}, size {wrong_size}, countries {countries:?}");
            }
            for wrong_type in [0, if v2 { 11 } else { 36 }] {
                let mut bytes = download(v2, countries);
                bytes[offset + 2] = wrong_type;
                assert!(parse_from_memory(&bytes).is_err(), "v2 {v2}, type {wrong_type}, countries {countries:?}");
            }
        }
    }
}

#[test]
fn overview_import_rejects_every_truncated_length() {
    for v2 in [false, true] {
        let bytes = download(v2, &[13, 15]);
        for length in 0..bytes.len() {
            assert!(parse_from_memory(&bytes[..length]).is_err(), "v2 {v2}, length {length}");
        }
    }
}
