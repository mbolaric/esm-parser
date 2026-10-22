use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::{
    WorkshopCardCalibrationAddData, WorkshopCardCalibrationAddDataParams, WorkshopCardCalibrationAddDataRecord,
};
use esm_parser::tacho::TimeReal;
use esm_parser::{Export, Readable, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::json;

fn section(bytes: &mut Vec<u8>, id: u16, appendix: u8, payload: &[u8]) {
    bytes.extend(id.to_be_bytes());
    bytes.push(appendix);
    bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
    bytes.extend(payload);
}

fn ring(count: usize, size: usize, pointer: u16, records: &[(usize, Vec<u8>)]) -> Vec<u8> {
    let mut bytes = pointer.to_be_bytes().to_vec();
    bytes.resize(2 + count * size, 0);
    for (index, record) in records {
        assert_eq!(record.len(), size);
        bytes[2 + index * size..2 + (index + 1) * size].copy_from_slice(record);
    }
    bytes
}

fn additional_record(marker: u8) -> Vec<u8> {
    let mut bytes = (1000 + u32::from(marker)).to_be_bytes().to_vec();
    bytes.extend(format!("TESTVIN{marker:010}").as_bytes());
    bytes.extend([marker, if marker == 1 { 13 } else { 15 }]);
    bytes.extend((900 + u32::from(marker)).to_be_bytes());
    assert_eq!(bytes.len(), 27);
    bytes
}

fn calibration() -> Vec<u8> {
    let mut bytes = vec![0, 2, 0, 254];
    for index in 0..255 {
        let mut record = vec![0; 178];
        record[1..18].fill(b' ');
        if index == 1 || index == 254 {
            let marker = if index == 1 { 1_u8 } else { 2 };
            record[0] = 4;
            record[1..18].copy_from_slice(format!("TESTVIN{marker:010}").as_bytes());
            record[61..65].copy_from_slice(&(1000 + u32::from(marker)).to_be_bytes());
        }
        bytes.extend(record);
    }
    bytes
}

fn payloads() -> Vec<(u16, Vec<u8>)> {
    let mut app = Vec::new();
    for capacity in [8_u16, 4, 8, 4, 3072] {
        app.extend(capacity.to_be_bytes());
    }
    let mut control = vec![0xC0];
    control.extend(2000_u32.to_be_bytes());
    control.extend([3, 13]);
    control.extend(b"TESTCONTROL00123");
    control.extend([13, 1]);
    control.extend(b"E-TEST-123   ");
    control.extend(1000_u32.to_be_bytes());
    control.extend(1900_u32.to_be_bytes());
    assert_eq!(control.len(), 46);
    let border = vec![13, 17, 0, 0, 0, 100, 1, 0, 0xB9, 0x94, 0, 0x0D, 6, 1, 1, 0xE2, 0x40];
    let load = vec![0, 0, 0, 200, 3, 0, 0, 0, 190, 7, 0, 0x0D, 6, 0, 0xB9, 0x94, 1, 9, 0xFB, 0xF1];
    let mut additional = ring(255, 27, 253, &[]);
    for record in additional[2..].as_chunks_mut::<27>().0 {
        record[4..21].fill(b' ');
    }
    additional[2..29].copy_from_slice(&additional_record(1));
    additional[2 + 253 * 27..2 + 254 * 27].copy_from_slice(&additional_record(2));
    vec![
        (0x0525, app),
        (0x0508, control),
        (0x0528, ring(4, 17, 3, &[(3, border)])),
        (0x0529, ring(8, 20, 7, &[(7, load)])),
        (0x0531, additional),
    ]
}

fn card(combined: bool, files: &[(u16, Vec<u8>)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    if combined {
        section(&mut bytes, 0x0501, 0, &[2, 0, 0, 3, 1, 1, 236, 0, 4, 3, 88]);
    }
    let mut app = vec![2, 1, if files.iter().any(|(id, _)| *id == 0x0525) { 1 } else { 0 }, 3, 6];
    for capacity in [492_u16, 8, 8, 255, 24, 4, 8] {
        app.extend(capacity.to_be_bytes());
    }
    section(&mut bytes, 0x0501, 2, &app);
    section(&mut bytes, 0x050A, 2, &calibration());
    for (id, payload) in files {
        section(&mut bytes, *id, 2, payload);
        section(&mut bytes, *id, 3, &[0x55; 64]);
    }
    bytes
}

#[test]
fn workshop_v2_files_are_decoded_in_gen2_only_and_combined_downloads() {
    for combined in [false, true] {
        let files = payloads();
        let parsed = parse_from_memory(&card(combined, &files)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let workshop = &value["cardDataResponses"]["gen2"];
        assert_eq!(
            workshop["applicationIdentificationV2"],
            json!({"lengthOfFollowingData":8,"noOfBorderCrossingRecords":4,"noOfLoadUnloadRecords":8,"noOfLoadTypeEntryRecords":4,"vuConfigurationLengthRange":3072})
        );
        let control = &workshop["controlActivityData"];
        assert_eq!(control["controlType"], 0xC0);
        assert_eq!(control["controlTime"], json!(TimeReal::new(2000)));
        assert_eq!(control["controlCardNumber"]["cardType"], "ControlCard");
        assert_eq!(control["controlVehicleRegistration"]["vehicleRegistrationNumber"], "E-TEST-123");
        let borders = &workshop["borderCrossings"];
        assert_eq!(borders["borderCrossingPointerNewestRecord"], 3);
        assert_eq!(borders["cardBorderCrossingRecords"].as_array().unwrap().len(), 1);
        assert_eq!(borders["cardBorderCrossingRecords"][0]["countryEntered"], "France");
        let loads = &workshop["loadUnloadOperations"];
        assert_eq!(loads["loadUnloadPointerNewestRecord"], 7);
        assert_eq!(loads["cardLoadUnloadRecords"].as_array().unwrap().len(), 1);
        assert_eq!(loads["cardLoadUnloadRecords"][0]["operationType"], "SimultaneousLoadAndUnload");
        assert_eq!(loads["cardLoadUnloadRecords"][0]["vehicleOdometerValue"], 654321);
        let extra = &workshop["calibrationAddData"];
        assert_eq!(extra["calibrationPointerNewestRecord"], 253);
        let records = extra["workshopCardCalibrationAddDataRecords"].as_array().unwrap();
        assert_eq!(records.len(), 255);
        for (index, marker, country, load) in [(0, 1, "Germany", "Goods"), (253, 2, "Spain", "Passengers")] {
            let record = &records[index];
            assert_eq!(record["oldTimeValue"], json!(TimeReal::new(1000 + marker)));
            assert_eq!(record["vehicleIdentificationNumber"], format!("TESTVIN{marker:010}"));
            assert_eq!(record["byDefaultLoadType"], load);
            assert_eq!(record["calibrationCountry"], country);
            assert_eq!(record["calibrationCountryTimestamp"], json!(TimeReal::new(900 + marker)));
            let matches = workshop["calibration"]["calibrationRecords"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|main| {
                    main["oldTimeValue"] == record["oldTimeValue"]
                        && main["vehicleIdentificationNumber"] == record["vehicleIdentificationNumber"]
                })
                .count();
            assert_eq!(matches, 1, "Associate by old time and VIN, not physical index or pointer");
        }
        assert_eq!(records[1]["oldTimeValue"], json!(TimeReal::new(0)));
        assert_eq!(records[1]["vehicleIdentificationNumber"], "");
        for (id, payload) in &files {
            let key = esm_parser::tacho::CardFileID::from(*id).to_string();
            assert_eq!(workshop["dataFiles"][&key]["data"], json!(payload));
            assert_eq!(workshop["dataFiles"][&key]["signature"], json!(vec![0x55; 64]));
        }
        assert!(parsed.to_xml().unwrap().contains("<calibrationCountry>Spain</calibrationCountry>"));
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}

#[test]
fn missing_workshop_v2_files_remain_optional() {
    for combined in [false, true] {
        let value = serde_json::to_value(parse_from_memory(&card(combined, &[])).unwrap()).unwrap();
        let workshop = &value["cardDataResponses"]["gen2"];
        for field in [
            "applicationIdentificationV2",
            "controlActivityData",
            "borderCrossings",
            "loadUnloadOperations",
            "calibrationAddData",
        ] {
            assert!(workshop.get(field).unwrap().is_null(), "{field}");
        }
    }
}

#[test]
fn workshop_border_and_load_files_require_v2_capacities() {
    for id in [0x0528, 0x0529] {
        let file = payloads().into_iter().find(|(kind, _)| *kind == id).unwrap();
        let Err(error) = parse_from_memory(&card(false, &[file])) else {
            panic!("Missing V2 capacities must be rejected");
        };
        assert!(matches!(error, esm_parser::Error::MissingCardFile(message) if message.contains("ApplicationIdentificationV2")));
    }
}

#[test]
fn new_workshop_dispatch_rejects_truncated_files_and_invalid_ring_pointers() {
    for id in [0x0525, 0x0508, 0x0528, 0x0529, 0x0531] {
        let files = payloads();
        let mut invalid = files.clone();
        invalid.iter_mut().find(|(kind, _)| *kind == id).unwrap().1.pop();
        assert!(parse_from_memory(&card(false, &invalid)).is_err(), "truncated {id:04X}");
        if let Some(capacity) = match id {
            0x0528 => Some(4_u16),
            0x0529 => Some(8),
            0x0531 => Some(255),
            _ => None,
        } {
            let mut invalid = files;
            invalid.iter_mut().find(|(kind, _)| *kind == id).unwrap().1[..2].copy_from_slice(&capacity.to_be_bytes());
            assert!(parse_from_memory(&card(false, &invalid)).is_err(), "pointer {id:04X}");
        }
    }
}

#[test]
fn additional_calibration_records_consume_exactly_27_bytes_and_reject_truncation_and_invalid_vins() {
    let mut bytes = additional_record(1);
    bytes.extend(additional_record(2));
    bytes.push(0xAA);
    let mut reader = BinMemoryBuffer::from(bytes);
    for marker in 1_u8..=2 {
        let record = WorkshopCardCalibrationAddDataRecord::read(&mut reader).unwrap();
        assert_eq!(record.old_time_value.data, 1000 + u32::from(marker));
        assert_eq!(record.vehicle_identification_number, format!("TESTVIN{marker:010}"));
        assert_eq!(reader.pos().unwrap(), usize::from(marker) * 27);
    }
    assert_eq!(reader.read_u8().unwrap(), 0xAA);
    for length in 0..27 {
        let bytes = additional_record(1);
        assert!(WorkshopCardCalibrationAddDataRecord::read(&mut BinMemoryBuffer::from(&bytes[..length])).is_err());
    }
    let mut bytes = additional_record(1);
    bytes[4] = 0x80;
    assert!(WorkshopCardCalibrationAddDataRecord::read(&mut BinMemoryBuffer::from(bytes)).is_err());
}

#[test]
fn additional_calibration_arrays_validate_capacity_pointer_and_exact_ef_size_and_retain_default_slots() {
    let mut empty = vec![0; 27];
    empty[4..21].fill(b' ');
    let valid = ring(2, 27, 1, &[(0, empty), (1, additional_record(1))]);
    let mut reader = BinMemoryBuffer::from(valid.clone());
    let parsed = WorkshopCardCalibrationAddData::read(&mut reader, &WorkshopCardCalibrationAddDataParams::new(2)).unwrap();
    assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    assert_eq!(parsed.workshop_card_calibration_add_data_records.len(), 2);
    assert_eq!(parsed.workshop_card_calibration_add_data_records[0].old_time_value.data, 0);
    assert_eq!(parsed.workshop_card_calibration_add_data_records[0].vehicle_identification_number, "");
    for length in (0..valid.len()).chain([valid.len() + 1]) {
        let mut bytes = valid.clone();
        bytes.resize(length, 0);
        assert!(
            WorkshopCardCalibrationAddData::read(
                &mut BinMemoryBuffer::from(bytes),
                &WorkshopCardCalibrationAddDataParams::new(2)
            )
            .is_err()
        );
    }
    for count in [0, 256, u16::MAX] {
        assert!(
            WorkshopCardCalibrationAddData::read(
                &mut BinMemoryBuffer::from(valid.clone()),
                &WorkshopCardCalibrationAddDataParams::new(count)
            )
            .is_err()
        );
    }
    for pointer in [2_u16, u16::MAX] {
        let mut bytes = valid.clone();
        bytes[..2].copy_from_slice(&pointer.to_be_bytes());
        assert!(
            WorkshopCardCalibrationAddData::read(
                &mut BinMemoryBuffer::from(bytes),
                &WorkshopCardCalibrationAddDataParams::new(2)
            )
            .is_err()
        );
    }
}
