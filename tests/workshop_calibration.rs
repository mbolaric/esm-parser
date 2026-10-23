#[path = "common/card.rs"]
mod card;

use binary_data::{BinMemoryBuffer, BinSeek};
use card::section;
use esm_parser::tacho::{CalibrationPurpose, CardGeneration, WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams};
use esm_parser::{Export, Readable, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn record(gen2: bool, index: u32, seals: u8) -> Vec<u8> {
    let mut bytes = vec![0; if gen2 { 178 } else { 105 }];
    bytes[0] = if index == 1 {
        4
    } else if index == 2 {
        6
    } else {
        0
    };
    bytes[1..18].copy_from_slice(format!("SYNTHETIC{index:08}").as_bytes());
    bytes[18] = 13;
    bytes[19] = 1;
    bytes[20..33].copy_from_slice(format!("TEST{index:09}").as_bytes());
    for offset in [33, 35, 37] {
        bytes[offset..offset + 2].copy_from_slice(&(1000 + index as u16).to_be_bytes());
    }
    bytes[39..54].fill(b' ');
    bytes[54] = 90;
    bytes[55..58].copy_from_slice(&(100000 + index).to_be_bytes()[1..]);
    bytes[58..61].copy_from_slice(&(200000 + index).to_be_bytes()[1..]);
    for (offset, value) in [(61, 1000 + index), (65, 2000 + index), (69, 3000 + index)] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
    bytes[73..89].copy_from_slice(format!("PART{index:012}").as_bytes());
    for offset in if gen2 { vec![89, 97, 105, 113] } else { vec![89, 97] } {
        bytes[offset..offset + 4].copy_from_slice(&(10000 + offset as u32 + index).to_be_bytes());
        bytes[offset + 4..offset + 6].copy_from_slice(&[0x06, 0x24]);
    }
    if gen2 {
        bytes[121] = seals;
        for seal in 0..seals {
            let start = 122 + usize::from(seal) * 11;
            bytes[start] = 1;
            bytes[start + 1..start + 3].copy_from_slice(b"TT");
            bytes[start + 3..start + 11].copy_from_slice(format!("{index:04}{seal:04}").as_bytes());
        }
        // TCS_162 allocates a complete 178-byte physical slot, including unused tail bytes.
        bytes[122 + usize::from(seals) * 11..].fill(0xA5);
    }
    bytes
}

fn calibration(gen2: bool, capacity: u16, pointer: u16) -> Vec<u8> {
    let mut bytes = 500_u16.to_be_bytes().to_vec();
    if gen2 {
        bytes.extend(pointer.to_be_bytes());
    } else {
        bytes.push(u8::try_from(pointer).unwrap());
    }
    for index in 0..capacity {
        let (marker, seals) = if index == 0 {
            (1, 1)
        } else if index == capacity - 1 {
            (2, 5)
        } else {
            (0, 0)
        };
        bytes.extend(record(gen2, marker, seals));
    }
    bytes
}

fn card(gen1: bool, gen2_version: Option<u8>) -> Vec<u8> {
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    if gen1 {
        let mut app = vec![2, 0, 0, 3, 1];
        app.extend(492_u16.to_be_bytes());
        app.extend(4_u16.to_be_bytes());
        app.extend([3, 88]);
        section(&mut bytes, 0x0501, 0, &app);
        section(&mut bytes, 0x050A, 0, &calibration(false, 88, 87));
        section(&mut bytes, 0x050A, 1, &[0x11; 128]);
    }
    if let Some(version) = gen2_version {
        let mut app = vec![2, 1, version, 3, 6];
        for capacity in [492_u16, 8, 8, 255, 24, 4, 8] {
            app.extend(capacity.to_be_bytes());
        }
        section(&mut bytes, 0x0501, 2, &app);
        section(&mut bytes, 0x050A, 2, &calibration(true, 255, 254));
        section(&mut bytes, 0x050A, 3, &[0x22; 64]);
    }
    bytes
}

fn assert_calibration(data: &Value, gen2: bool, capacity: usize) {
    let calibration = &data["calibration"];
    assert_eq!(calibration["calibrationTotalNumber"], 500);
    assert_eq!(calibration["calibrationPointerNewestRecord"], capacity - 1);
    let records = calibration["calibrationRecords"].as_array().unwrap();
    assert_eq!(records.len(), capacity);
    for (record, marker, purpose, seals) in
        [(&records[0], 1, "PeriodicInspection", 1), (&records[capacity - 1], 2, "TimeAdjustmentWithoutCalibration", 5)]
    {
        assert_eq!(record["calibrationPurpose"], purpose);
        assert_eq!(record["vehicleIdentificationNumber"], format!("SYNTHETIC{marker:08}"));
        assert_eq!(record["vehicleRegistration"]["vehicleRegistrationNumber"], format!("TEST{marker:09}"));
        assert_eq!(record["oldOdometerValue"], 100000 + marker);
        assert_eq!(record["vuSerialNumber"]["serialNumber"], 10089 + marker);
        if gen2 {
            assert_eq!(record["sensorGnssSerialNumber"]["serialNumber"], 10105 + marker);
            assert_eq!(record["rcmSerialNumber"]["serialNumber"], 10113 + marker);
            assert_eq!(record["sealDataCard"]["noOfSealRecords"], seals);
            assert_eq!(record["sealDataCard"]["sealRecords"].as_array().unwrap().len(), seals);
        }
    }
    let raw = &data["dataFiles"]["Calibration"];
    assert_eq!(raw["data"], json!(self::calibration(gen2, capacity as u16, (capacity - 1) as u16)));
    assert_eq!(raw["signature"], if gen2 { json!(vec![0x22; 64]) } else { json!(vec![0x11; 128]) });
}

#[test]
fn workshop_calibrations_parse_in_gen1_gen2_v1_v2_and_combined_downloads() {
    for (gen1, gen2_version) in [(true, None), (false, Some(0)), (false, Some(1)), (true, Some(1))] {
        let parsed = parse_from_memory(&card(gen1, gen2_version)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let response = &value["cardDataResponses"];
        if gen2_version.is_some() {
            assert_calibration(&response["gen2"], true, 255);
            if gen1 {
                assert_calibration(&response["gen1"], false, 88);
            }
        } else {
            assert_calibration(response, false, 88);
        }
        assert!(parsed.to_xml().unwrap().contains("<calibrationPointerNewestRecord>"));
        let envelope = serde_json::to_value(SerializedTachographData::from(parsed)).unwrap();
        assert_eq!(envelope["data"], value);
    }
}

#[test]
fn gen1_calibrations_have_a_one_byte_pointer_and_exact_record_boundaries() {
    let mut reader = BinMemoryBuffer::from(calibration(false, 2, 1));
    let data = WorkshopCardCalibrationData::<esm_parser::gen1::WorkshopCardCalibrationRecord>::read(
        &mut reader,
        &WorkshopCardCalibrationDataParams::new(2, CardGeneration::Gen1),
    )
    .unwrap();
    assert_eq!(data.calibration_pointer_newest_record, 1);
    assert_eq!(data.calibration_records[0].calibration_purpose, CalibrationPurpose::PeriodicInspection);
    assert_eq!(data.calibration_records[1].calibration_purpose, CalibrationPurpose::TimeAdjustmentWithoutCalibration);
    assert_eq!(reader.pos().unwrap(), 3 + 2 * 105);
    assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
}

#[test]
fn gen2_calibrations_have_a_two_byte_pointer_and_fixed_slots_for_every_seal_count() {
    for seals in 0..=5 {
        let mut bytes = 500_u16.to_be_bytes().to_vec();
        bytes.extend(1_u16.to_be_bytes());
        bytes.extend(record(true, 1, seals));
        bytes.extend(record(true, 2, 5));
        let mut reader = BinMemoryBuffer::from(bytes);
        let data = WorkshopCardCalibrationData::<esm_parser::gen2::WorkshopCardCalibrationRecord>::read(
            &mut reader,
            &WorkshopCardCalibrationDataParams::new(2, CardGeneration::Gen2),
        )
        .unwrap();
        assert_eq!(data.calibration_pointer_newest_record, 1);
        assert_eq!(data.calibration_records[0].calibration_purpose, CalibrationPurpose::PeriodicInspection);
        assert_eq!(data.calibration_records[0].seal_data_card.seal_records.len(), usize::from(seals));
        assert_eq!(data.calibration_records[1].calibration_purpose, CalibrationPurpose::TimeAdjustmentWithoutCalibration);
        assert_eq!(data.calibration_records[1].vehicle_identification_number, "SYNTHETIC00000002");
        assert_eq!(reader.pos().unwrap(), 4 + 2 * 178);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }
}

fn read_calibration(bytes: Vec<u8>, generation: CardGeneration, count: u16) -> esm_parser::Result<()> {
    let mut reader = BinMemoryBuffer::from(bytes);
    let params = WorkshopCardCalibrationDataParams::new(count, generation);
    if generation == CardGeneration::Gen1 {
        WorkshopCardCalibrationData::<esm_parser::gen1::WorkshopCardCalibrationRecord>::read(&mut reader, &params).map(|_| ())
    } else {
        WorkshopCardCalibrationData::<esm_parser::gen2::WorkshopCardCalibrationRecord>::read(&mut reader, &params).map(|_| ())
    }
}

#[test]
fn calibration_containers_reject_bad_boundaries_pointers_capacities_and_generation() {
    for generation in [CardGeneration::Gen1, CardGeneration::Gen2] {
        let gen2 = generation == CardGeneration::Gen2;
        let valid = calibration(gen2, 2, 1);
        for length in [0, 1, 2, 3, valid.len() - 1, valid.len() + 1] {
            let mut bytes = valid.clone();
            bytes.resize(length, 0);
            assert!(read_calibration(bytes, generation, 2).is_err());
        }
        for pointer in [2, 255] {
            assert!(matches!(
                read_calibration(calibration(gen2, 2, pointer), generation, 2),
                Err(esm_parser::Error::RecordOutOfRange(_))
            ));
        }
        for count in [0, 256, u16::MAX] {
            assert!(matches!(read_calibration(valid.clone(), generation, count), Err(esm_parser::Error::RecordOutOfRange(_))));
        }
    }
    let mut bytes = calibration(true, 2, 1);
    bytes.remove(2);
    assert!(read_calibration(bytes, CardGeneration::Gen2, 2).is_err());
    assert!(read_calibration(calibration(true, 2, 1), CardGeneration::Combined, 2).is_err());
    let mut bytes = calibration(true, 2, 1);
    bytes[4 + 121] = 6;
    assert!(matches!(read_calibration(bytes, CardGeneration::Gen2, 2), Err(esm_parser::Error::RecordOutOfRange(_))));
}

#[test]
fn standalone_card_seal_lists_remain_counted_and_reject_more_than_five_seals() {
    for count in [0, 1, 5] {
        let mut bytes = vec![count];
        bytes.resize(1 + usize::from(count) * 11, 0);
        let mut reader = BinMemoryBuffer::from(bytes);
        let data = esm_parser::gen2::SealDataCard::read(&mut reader).unwrap();
        assert_eq!(data.seal_records.len(), usize::from(count));
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }
    let mut reader = BinMemoryBuffer::from(vec![6]);
    assert!(matches!(esm_parser::gen2::SealDataCard::read(&mut reader), Err(esm_parser::Error::RecordOutOfRange(_))));
    assert_eq!(reader.pos().unwrap(), 1);
}
