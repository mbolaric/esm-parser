#[path = "common/card.rs"]
mod card;

use card::section;
use esm_parser::{Error, Export, SerializedTachographData, parse_from_memory};
use serde_json::{Value, json};

fn application(kind: u8, gen2: bool) -> Vec<u8> {
    let mut bytes = vec![kind, u8::from(gen2), 0];
    if kind == 3 || kind == 4 {
        bytes.extend(230_u16.to_be_bytes());
        return bytes;
    }
    let (events, faults, activity, vehicles, places) = match (kind, gen2) {
        (2, false) => (3, 1, 492_u16, 4_u16, 3_u16),
        (2, true) => (3, 6, 492, 8, 8),
        (_, false) => (6, 12, 13776, 200, 112),
        (_, true) => (12, 24, 13776, 200, 112),
    };
    bytes.extend([events, faults]);
    bytes.extend(activity.to_be_bytes());
    bytes.extend(vehicles.to_be_bytes());
    if gen2 {
        bytes.extend(places.to_be_bytes());
        if kind == 2 {
            bytes.extend(255_u16.to_be_bytes());
        }
        for capacity in if kind == 2 { [24_u16, 4, 8] } else { [336_u16, 112, 200] } {
            bytes.extend(capacity.to_be_bytes());
        }
    } else {
        bytes.push(u8::try_from(places).unwrap());
        if kind == 2 {
            bytes.push(88);
        }
    }
    bytes
}

fn card(kind: u8, gen1: bool, gen2: bool) -> Vec<u8> {
    // Partial downloads isolate signature framing without requiring real certificates or personal data.
    let mut bytes = Vec::new();
    section(&mut bytes, 0x0002, 0, &[0; 25]);
    section(&mut bytes, 0x0005, 0, &[0; 8]);
    for (enabled, appendix) in [(gen1, 0), (gen2, 2)] {
        if enabled {
            section(&mut bytes, 0x0501, appendix, &application(kind, appendix == 2));
        }
    }
    bytes
}

fn generation(value: &Value, appendix: u8, gen2: bool) -> &Value {
    let responses = &value["cardDataResponses"];
    if appendix == 2 {
        &responses["gen2"]
    } else if gen2 {
        &responses["gen1"]
    } else {
        responses
    }
}

#[test]
fn duplicate_signatures_are_rejected_for_every_card_type_and_application() {
    for kind in 1..=4 {
        for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
            for (enabled, appendix, size) in [(gen1, 0, 128), (gen2, 2, 64)] {
                if !enabled {
                    continue;
                }
                for second in [0x11, 0x22] {
                    let mut bytes = card(kind, gen1, gen2);
                    section(&mut bytes, 0x0501, appendix + 1, &vec![0x11; size]);
                    section(&mut bytes, 0x0501, appendix + 1, &vec![second; size]);
                    for wrapped in [false, true] {
                        let download = if wrapped { [vec![0x76, 0x06], bytes.clone()].concat() } else { bytes.clone() };
                        assert!(
                            matches!(parse_from_memory(&download), Err(Error::DuplicateCardFile)),
                            "card type {kind}, appendix {appendix}, wrapped {wrapped}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn unsigned_and_single_signature_downloads_preserve_every_application() {
    for kind in 1..=4 {
        for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
            for signed in [false, true] {
                let mut bytes = card(kind, gen1, gen2);
                if signed {
                    for (enabled, appendix, size, marker) in [(gen1, 0, 128, 0x11), (gen2, 2, 64, 0x22)] {
                        if enabled {
                            section(&mut bytes, 0x0501, appendix + 1, &vec![marker; size]);
                        }
                    }
                }
                let parsed = parse_from_memory(&bytes).unwrap();
                let value = serde_json::to_value(&parsed).unwrap();
                for (enabled, appendix, size, marker) in [(gen1, 0, 128, 0x11), (gen2, 2, 64, 0x22)] {
                    if !enabled {
                        continue;
                    }
                    let raw = &generation(&value, appendix, gen2)["dataFiles"]["ApplicationIdentification"];
                    assert_eq!(raw["data"], json!(application(kind, appendix == 2)));
                    assert_eq!(raw["signature"], if signed { json!(vec![marker; size]) } else { Value::Null });
                }
                parsed.to_xml().unwrap();
                assert_eq!(serde_json::to_value(SerializedTachographData::from(parsed)).unwrap()["data"], value);
            }
        }
    }
}

#[test]
fn signatures_are_independent_for_different_files_and_generations() {
    for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
        let mut bytes = card(1, gen1, gen2);
        for (enabled, appendix, size, marker) in [(gen1, 0, 128, 0x11), (gen2, 2, 64, 0x22)] {
            if !enabled {
                continue;
            }
            section(&mut bytes, 0x0501, appendix + 1, &vec![marker; size]);
            section(&mut bytes, 0x050E, appendix, &1_700_000_000_u32.to_be_bytes());
            section(&mut bytes, 0x050E, appendix + 1, &vec![marker + 1; size]);
        }
        let parsed = serde_json::to_value(parse_from_memory(&bytes).unwrap()).unwrap();
        for (enabled, appendix, size, marker) in [(gen1, 0, 128, 0x11), (gen2, 2, 64, 0x22)] {
            if enabled {
                let files = &generation(&parsed, appendix, gen2)["dataFiles"];
                assert_eq!(files["ApplicationIdentification"]["signature"], json!(vec![marker; size]));
                assert_eq!(files["CardDownload"]["signature"], json!(vec![marker + 1; size]));
            }
        }
        for (enabled, appendix, size) in [(gen1, 0, 128), (gen2, 2, 64)] {
            if enabled {
                let mut duplicate = bytes.clone();
                section(&mut duplicate, 0x0501, appendix + 1, &vec![0x33; size]);
                assert!(matches!(parse_from_memory(&duplicate), Err(Error::DuplicateCardFile)));
            }
        }
    }
}

#[test]
fn empty_signature_sections_are_rejected_before_they_can_replace_data() {
    for (appendix, size) in [(0, 128), (2, 64)] {
        for (first, second) in [(vec![], vec![0x22; size]), (vec![0x11; size], vec![]), (vec![], vec![])] {
            let mut bytes = card(1, appendix == 0, appendix == 2);
            section(&mut bytes, 0x0501, appendix + 1, &first);
            section(&mut bytes, 0x0501, appendix + 1, &second);
            match parse_from_memory(&bytes) {
                Err(Error::DuplicateCardFile) => (),
                Err(Error::InvalidDataParse(message)) => assert!(message.contains("Signature") && message.contains("size")),
                _ => panic!("An empty signature must not be accepted"),
            }
        }
    }
}

#[test]
fn duplicate_data_and_signatures_before_their_own_generation_data_remain_errors() {
    for appendix in [0, 2] {
        let mut bytes = card(1, appendix == 0, appendix == 2);
        section(&mut bytes, 0x0501, appendix, &application(1, appendix == 2));
        assert!(matches!(parse_from_memory(&bytes), Err(Error::DuplicateCardFile)));

        let mut bytes = card(1, appendix == 2, appendix == 0);
        section(&mut bytes, 0x0501, appendix + 1, &[0x11; 64]);
        section(&mut bytes, 0x0501, appendix, &application(1, appendix == 2));
        assert!(matches!(parse_from_memory(&bytes), Err(Error::SignatureBeforeCardFile)));
    }
}
