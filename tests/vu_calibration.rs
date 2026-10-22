use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use esm_parser::gen2::{VUTechnicalData, VuCalibrationRecord};
use esm_parser::tacho::{RecordType, VUTransferResponseParameterID};
use esm_parser::{Export, ReadableWithParams, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn text(bytes: &mut Vec<u8>, value: &str, length: usize) {
    bytes.extend(value.as_bytes());
    bytes.resize(bytes.len() + length - value.len(), b' ');
}

fn calibration_record(v2: bool, marker: u8) -> Vec<u8> {
    let mut bytes = vec![4];
    for value in [format!("TEST WORKSHOP {marker}"), format!("TEST ADDRESS {marker}")] {
        bytes.push(1);
        text(&mut bytes, &value, 35);
    }
    bytes.extend([2, 13]);
    text(&mut bytes, &format!("TEST{marker:012}"), 16);
    bytes.extend((1_800_000_000 + u32::from(marker)).to_be_bytes());
    text(&mut bytes, &format!("SYNTHETIC{marker:08}"), 17);
    bytes.extend([13, 1]);
    text(&mut bytes, &format!("TEST{marker:09}"), 13);
    for value in [1000_u16, 2000, 3000] {
        bytes.extend((value + u16::from(marker)).to_be_bytes());
    }
    text(&mut bytes, "TEST TYRE", 15);
    bytes.push(90 + marker);
    for value in [123_456_u32, 234_567] {
        bytes.extend((value + u32::from(marker)).to_be_bytes()[1..].iter());
    }
    for value in [1_700_000_000_u32, 1_700_000_100, 1_760_000_000] {
        bytes.extend((value + u32::from(marker)).to_be_bytes());
    }
    assert_eq!(bytes.len(), 167);
    if v2 {
        for serial in 1..=3_u8 {
            bytes.extend((1000 * u32::from(marker) + u32::from(serial)).to_be_bytes());
            bytes.extend([0x06, 0x24, serial, 0x10 + marker]);
        }
    }
    for seal in 0..5_u8 {
        bytes.push(7);
        bytes.extend(*b"TT");
        text(&mut bytes, &format!("{marker:04}{seal:04}"), 8);
    }
    if v2 {
        bytes.extend([marker, if marker == 1 { 1 } else { 13 }]);
        bytes.extend((1_700_000_200 + u32::from(marker)).to_be_bytes());
    }
    assert_eq!(bytes.len(), if v2 { 252 } else { 222 });
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
    // Isolate calibration from the separate identification/map-version finding (F07).
    for (kind, size) in [
        (RecordType::VuIdentification, if v2 { 138 } else { 126 }),
        (RecordType::SensorPairedRecord, 28),
        (RecordType::SensorExternalGNSSCoupledRecord, 28),
    ] {
        array(&mut bytes, kind, size, &[]);
    }
    array(
        &mut bytes,
        RecordType::VuCalibrationRecord,
        if v2 { 252 } else { 222 },
        &[calibration_record(v2, 1), calibration_record(v2, 2)],
    );
    for (kind, size) in
        [(RecordType::VuCardRecord, 45), (RecordType::VuITSConsentRecord, 20), (RecordType::VuPowerSupplyInterruptionRecord, 87)]
    {
        array(&mut bytes, kind, size, &[]);
    }
    array(&mut bytes, RecordType::Signature, 64, &[vec![0x55; 64]]);
    bytes
}

fn download(v2: bool) -> Vec<u8> {
    // A partial overview supplies the download header; this fixture tests binary layout, not certification.
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

fn assert_record(value: &Value, v2: bool, marker: u8) {
    assert_eq!(value["isGen2V2"], v2);
    assert_eq!(value["calibrationPurpose"], "PeriodicInspection");
    assert_eq!(value["workshopName"], format!("TEST WORKSHOP {marker}"));
    assert_eq!(value["workshopAddress"], format!("TEST ADDRESS {marker}"));
    assert_eq!(value["workshopCardNumber"]["cardType"], "WorkshopCard");
    assert_eq!(value["workshopCardNumber"]["cardNumber"], format!("TEST{marker:012}"));
    assert_eq!(value["vehicleIdentificationNumber"], format!("SYNTHETIC{marker:08}"));
    assert_eq!(value["vehicleRegistrationIdentification"]["vehicleRegistrationNumber"], format!("TEST{marker:09}"));
    assert_eq!(value["wVehicleCharacteristicConstant"], 1000 + u16::from(marker));
    assert_eq!(value["kConstantOfRecordingEquipment"], 2000 + u16::from(marker));
    assert_eq!(value["lTyreCircumference"], 3000 + u16::from(marker));
    assert_eq!(value["tyreSize"], "TEST TYRE");
    assert_eq!(value["authorisedSpeed"], 90 + marker);
    assert_eq!(value["oldOdometerValue"], 123_456 + u32::from(marker));
    assert_eq!(value["newOdometerValue"], 234_567 + u32::from(marker));
    let seals = value["sealDataVu"]["sealRecords"].as_array().unwrap();
    assert_eq!(seals.len(), 5);
    for (index, seal) in seals.iter().enumerate() {
        assert_eq!(seal["equipmentType"], "MotionSensor");
        assert_eq!(seal["extendedSealIdentifier"]["manufacturerCode"], json!(b"TT"));
        assert_eq!(seal["extendedSealIdentifier"]["sealIdentifier"], json!(format!("{marker:04}{index:04}").as_bytes()));
    }
    for (index, field) in ["sensorSerialNumber", "sensorGnssSerialNumber", "rcmSerialNumber"].iter().enumerate() {
        if v2 {
            assert_eq!(value[field]["serialNumber"], 1000 * u32::from(marker) + index as u32 + 1);
            assert_eq!(value[field]["monthYear"], "0624");
            assert_eq!(value[field]["type"], index + 1);
            assert_eq!(value[field]["manufacturerCode"], 0x10 + marker);
        } else {
            assert!(value[field].is_null());
        }
    }
    if v2 {
        assert_eq!(value["byDefaultLoadType"], if marker == 1 { "Goods" } else { "Passengers" });
        assert_eq!(value["calibrationCountry"], if marker == 1 { "Austria" } else { "Germany" });
        assert_eq!(value["calibrationCountryTimestamp"], format!("2023-11-14 22:16:{:02} UTC", 40 + marker));
    } else {
        for field in ["byDefaultLoadType", "calibrationCountry", "calibrationCountryTimestamp"] {
            assert!(value[field].is_null());
        }
    }
}

#[test]
fn v1_and_v2_calibrations_decode_two_distinct_records_at_exact_boundaries() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let mut bytes = vec![0xCC; 3];
        bytes.extend(calibration_record(v2, 1));
        bytes.extend(calibration_record(v2, 2));
        bytes.extend([0xAA, 0xBB]);
        let mut reader = BinMemoryBuffer::from(bytes);
        reader.seek(3).unwrap();
        for marker in 1..=2 {
            let record = VuCalibrationRecord::read(&mut reader, &trep).unwrap();
            assert_record(&serde_json::to_value(record).unwrap(), v2, marker);
            assert_eq!(reader.pos().unwrap(), 3 + usize::from(marker) * if v2 { 252 } else { 222 });
        }
        assert_eq!(reader.read_bytes::<2>().unwrap(), [0xAA, 0xBB]);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }
}

#[test]
fn calibration_records_reject_every_truncated_length() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let bytes = calibration_record(v2, 1);
        for length in 0..bytes.len() {
            assert!(
                VuCalibrationRecord::read(&mut BinMemoryBuffer::from(&bytes[..length]), &trep).is_err(),
                "{trep:?}, length {length}"
            );
        }
    }
}

#[test]
fn technical_calibration_arrays_reject_incorrect_declared_record_sizes() {
    for (v2, trep) in
        [(false, VUTransferResponseParameterID::Gen2TechnicalData), (true, VUTransferResponseParameterID::Gen2v2TechnicalData)]
    {
        let size = if v2 { 252_u16 } else { 222 };
        for wrong in [0, size - 1, size + 1] {
            let mut bytes = technical_data(v2);
            bytes[16..18].copy_from_slice(&wrong.to_be_bytes());
            bytes[18..20].copy_from_slice(&1_u16.to_be_bytes());
            let mut record = calibration_record(v2, 1);
            record.resize(usize::from(wrong), 0);
            bytes.splice(20..20 + usize::from(size) * 2, record);
            assert!(VUTechnicalData::from_data(trep, &mut BinMemoryBuffer::from(bytes)).is_err(), "{trep:?}, size {wrong}");
        }
    }
}

#[test]
fn v2_optional_serial_defaults_do_not_hide_invalid_bcd_or_shift_the_next_record() {
    let trep = VUTransferResponseParameterID::Gen2v2TechnicalData;
    for (index, field) in ["sensorSerialNumber", "sensorGnssSerialNumber", "rcmSerialNumber"].iter().enumerate() {
        let start = 167 + index * 8;
        let mut bytes = calibration_record(true, 1);
        bytes[start..start + 8].fill(0xFF);
        bytes.extend(calibration_record(true, 2));
        let mut reader = BinMemoryBuffer::from(bytes);
        let record = VuCalibrationRecord::read(&mut reader, &trep).unwrap();
        let value = serde_json::to_value(record).unwrap();
        assert!(value[field].is_null());
        assert_eq!(value["byDefaultLoadType"], "Goods");
        assert_eq!(reader.pos().unwrap(), 252);
        let next = VuCalibrationRecord::read(&mut reader, &trep).unwrap();
        assert_record(&serde_json::to_value(next).unwrap(), true, 2);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());

        let mut bytes = calibration_record(true, 1);
        bytes[start + 4] = 0xFA;
        assert!(VuCalibrationRecord::read(&mut BinMemoryBuffer::from(bytes), &trep).is_err());
    }
}

#[test]
fn public_vu_import_preserves_calibrations_raw_data_signatures_and_exports() {
    for v2 in [false, true] {
        let parsed = parse_from_memory(&download(v2)).unwrap();
        let value = serde_json::to_value(&parsed).unwrap();
        let array = &value["transferResParams"][1]["data"]["Calibration"]["vuCalibrationRecordArray"];
        assert_eq!(array["recordSize"], if v2 { 252 } else { 222 });
        assert_eq!(array["noOfRecords"], 2);
        for marker in 1..=2 {
            assert_record(&array["records"][usize::from(marker - 1)], v2, marker);
        }
        let raw = &value["dataFiles"][1];
        let bytes = technical_data(v2);
        assert_eq!(raw["rawData"], json!(bytes));
        assert_eq!(raw["data"], json!(&bytes[..bytes.len() - 69]));
        assert_eq!(raw["signature"], json!(vec![0x55; 64]));
        let xml = parsed.to_xml().unwrap();
        if v2 {
            assert!(xml.contains("<byDefaultLoadType>Goods</byDefaultLoadType>"));
            assert!(xml.contains("<calibrationCountry>Germany</calibrationCountry>"));
            assert!(xml.contains("<sensorGnssSerialNumber>"));
        }
        assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
    }
}
