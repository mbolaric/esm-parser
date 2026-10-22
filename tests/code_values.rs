use binary_data::BinMemoryBuffer;
use esm_parser::tacho::{CardControlActivityDataRecord, CardCurrentUse, ControlType, TimeReal, VUTransferResponseParameterID};
use esm_parser::{Readable, ReadableWithParams, gen1, gen2};

fn record_bytes(first_byte: u8) -> BinMemoryBuffer {
    let mut bytes = vec![0; 256];
    bytes[0] = first_byte;
    BinMemoryBuffer::from(bytes)
}

#[test]
fn card_and_vu_readers_preserve_combined_control_flags() {
    for raw in [0xC0, 0x30, 0xF8, 0xFF] {
        let card = CardControlActivityDataRecord::read(&mut record_bytes(raw)).unwrap();
        let mut gen1_bytes = vec![1, raw];
        gen1_bytes.resize(257, 0);
        let gen1 = gen1::VuControlActivity::read(&mut BinMemoryBuffer::from(gen1_bytes)).unwrap();
        let gen2 = gen2::VuControlActivityRecord::read(&mut record_bytes(raw)).unwrap();
        for control in [card.control_type, gen1.vu_control_activities[0].control_type, gen2.control_type] {
            assert_eq!(control.bits(), raw);
            assert_eq!(serde_json::to_value(control).unwrap(), raw);
        }
    }
    assert!(ControlType::from(0xC0).contains(ControlType::CARD_DOWNLOADED | ControlType::VU_DOWNLOADED));
}

#[test]
fn vu_readers_export_manufacturer_fault_codes_and_standard_gen2_faults() {
    let gen1 = gen1::VuFaultRecord::read(&mut record_bytes(0x85)).unwrap();
    assert_eq!(serde_json::to_value(gen1).unwrap()["faultType"], 133);
    let gen2 = gen2::VuFaultRecord::read(&mut record_bytes(0x85)).unwrap();
    assert_eq!(serde_json::to_value(gen2).unwrap()["faultType"], 133);
    let gen2 = gen2::VuFaultRecord::read(&mut record_bytes(0x38)).unwrap();
    assert_eq!(serde_json::to_value(gen2).unwrap()["faultType"], "RCMRemoteCommunicationModuleFault");
}

#[test]
fn vu_readers_export_manufacturer_calibration_purposes() {
    let mut gen1_bytes = vec![1, 0x80];
    gen1_bytes.resize(257, 0);
    let gen1 = gen1::VUCalibrationData::read(&mut BinMemoryBuffer::from(gen1_bytes)).unwrap();
    assert_eq!(serde_json::to_value(gen1).unwrap()["calibrations"][0]["calibrationPurpose"], 128);
    let gen2 =
        gen2::VuCalibrationRecord::read(&mut record_bytes(0x80), &VUTransferResponseParameterID::Gen2TechnicalData).unwrap();
    assert_eq!(serde_json::to_value(gen2).unwrap()["calibrationPurpose"], 128);
}

#[test]
fn current_session_zero_is_absent_without_changing_generic_timestamps_or_raw_bytes() {
    for (timestamp, expected) in
        [(0_u32, serde_json::Value::Null), (u32::MAX, serde_json::Value::Null), (1, serde_json::json!("1970-01-01 00:00:01 UTC"))]
    {
        let mut bytes = timestamp.to_be_bytes().to_vec();
        bytes.extend_from_slice(&[1, 1]); // Austria, ISO-8859-1
        bytes.extend_from_slice(b"TEST         ");
        let record = CardCurrentUse::read(&mut BinMemoryBuffer::from(bytes.clone())).unwrap();
        assert_eq!(record.session_open_time.data, timestamp);
        assert_eq!(serde_json::to_value(&record).unwrap()["sessionOpenTime"], expected);

        #[cfg(feature = "write")]
        {
            use esm_parser::writer::Writable;

            let mut writer = BinMemoryBuffer::new();
            record.write(&mut writer).unwrap();
            assert_eq!(writer.as_slice(), bytes.as_slice());
        }
    }
    assert_eq!(serde_json::to_value(TimeReal::new(0)).unwrap(), "1970-01-01 00:00:00 UTC");
}
