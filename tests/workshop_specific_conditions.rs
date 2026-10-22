use esm_parser::tacho::TimeReal;
use esm_parser::{Export, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn section(bytes: &mut Vec<u8>, id: u16, appendix: u8, payload: &[u8]) {
    bytes.extend(id.to_be_bytes());
    bytes.push(appendix);
    bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
    bytes.extend(payload);
}

fn conditions(count: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    for index in 0..count {
        bytes.extend((1_700_000_000 + index).to_be_bytes());
        bytes.push(if index % 2 == 0 { 1 } else { 2 });
    }
    bytes
}

fn card(workshop: bool, combined: bool, gen1_conditions: Option<&[u8]>) -> Vec<u8> {
    // Partial downloads with dummy signatures isolate specific-condition capacity and dispatch.
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    let mut app = if workshop { vec![2, 0, 0, 3, 1] } else { vec![1, 0, 0, 6, 2] };
    app.extend(if workshop { 492_u16 } else { 13776 }.to_be_bytes());
    app.extend(if workshop { 4_u16 } else { 200 }.to_be_bytes());
    app.push(if workshop { 3 } else { 112 });
    if workshop {
        app.push(88);
    }
    section(&mut bytes, 0x0501, 0, &app);
    if let Some(payload) = gen1_conditions {
        section(&mut bytes, 0x0522, 0, payload);
        section(&mut bytes, 0x0522, 1, &[0x11; 128]);
    }
    if workshop {
        section(&mut bytes, 0x050E, 0, &7_u16.to_be_bytes());
    }
    if combined {
        let mut app = if workshop { vec![2, 1, 0, 3, 6] } else { vec![1, 1, 0, 12, 24] };
        for capacity in if workshop { vec![492_u16, 8, 8, 255, 24, 4, 8] } else { vec![13776_u16, 200, 112, 336, 112, 200] } {
            app.extend(capacity.to_be_bytes());
        }
        section(&mut bytes, 0x0501, 2, &app);
        if workshop {
            let mut gen2_conditions = 3_u16.to_be_bytes().to_vec();
            gen2_conditions.extend(conditions(4));
            section(&mut bytes, 0x0522, 2, &gen2_conditions);
            section(&mut bytes, 0x0522, 3, &[0x22; 64]);
        }
    }
    bytes
}

fn gen1(value: &Value, combined: bool) -> &Value {
    if combined { &value["cardDataResponses"]["gen1"] } else { &value["cardDataResponses"] }
}

#[test]
fn workshop_specific_conditions_read_two_records_in_standalone_and_combined_cards() {
    for combined in [false, true] {
        for (payload, expected_count) in [(conditions(2), 2), (vec![0; 10], 0)] {
            let parsed = parse_from_memory(&card(true, combined, Some(&payload))).unwrap();
            let value = serde_json::to_value(&parsed).unwrap();
            let workshop = gen1(&value, combined);
            let records = workshop["specificConditions"]["specificConditionRecords"].as_array().unwrap();
            assert_eq!(records.len(), expected_count);
            if expected_count == 2 {
                assert_eq!(
                    records[0],
                    json!({"entryTime":TimeReal::new(1_700_000_000),"specificConditionType":"OutOfScopeBegin"})
                );
                assert_eq!(records[1], json!({"entryTime":TimeReal::new(1_700_000_001),"specificConditionType":"OutOfScopeEnd"}));
            }
            assert_eq!(workshop["noOfCalibrationsSinceDownload"], 7);
            assert_eq!(workshop["dataFiles"]["SpecificConditions"]["data"], json!(payload));
            assert_eq!(workshop["dataFiles"]["SpecificConditions"]["signature"], json!(vec![0x11; 128]));
            if combined {
                let gen2 = &value["cardDataResponses"]["gen2"];
                assert_eq!(gen2["specificConditions"]["conditionPointerNewestRecord"], 3);
                assert_eq!(gen2["specificConditions"]["specificConditionRecords"].as_array().unwrap().len(), 4);
                assert_eq!(gen2["dataFiles"]["SpecificConditions"]["signature"], json!(vec![0x22; 64]));
            }
            let xml = parsed.to_xml().unwrap();
            assert!(xml.contains("<specificConditions"));
            if expected_count == 2 {
                assert!(xml.contains("<specificConditionType>OutOfScopeBegin</specificConditionType>"));
                assert!(xml.contains("<specificConditionType>OutOfScopeEnd</specificConditionType>"));
            }
            assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
        }
    }
}

#[test]
fn workshop_specific_conditions_are_optional_and_reject_every_truncated_ef_length() {
    for combined in [false, true] {
        let value = serde_json::to_value(parse_from_memory(&card(true, combined, None)).unwrap()).unwrap();
        assert!(gen1(&value, combined)["specificConditions"].is_null());
        let payload = conditions(2);
        for length in 0..10 {
            assert!(
                parse_from_memory(&card(true, combined, Some(&payload[..length]))).is_err(),
                "combined {combined}, length {length}"
            );
        }
    }
}

#[test]
fn gen1_driver_specific_conditions_still_read_all_56_records() {
    let payload = conditions(56);
    for combined in [false, true] {
        let value = serde_json::to_value(parse_from_memory(&card(false, combined, Some(&payload))).unwrap()).unwrap();
        let records = gen1(&value, combined)["specificConditions"]["specificConditionRecords"].as_array().unwrap();
        assert_eq!(records.len(), 56);
        assert_eq!(records[55]["entryTime"], json!(TimeReal::new(1_700_000_055)));
        assert_eq!(records[55]["specificConditionType"], "OutOfScopeEnd");
    }
}
