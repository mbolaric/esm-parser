use std::path::PathBuf;

use esm_parser::tacho::{TachographDataGeneration, TachographDataType, TachographHeader};
use esm_parser::{parse_from_file, parse_from_memory};
use serde_json::Value;

type Section = (u16, u8, Vec<u8>);

struct TemporaryFile(PathBuf);

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn sections(kind: u8, gen1: bool, gen2: bool) -> Vec<Section> {
    let mut sections = Vec::new();
    for (enabled, appendix) in [(gen1, 0), (gen2, 2)] {
        if !enabled {
            continue;
        }
        let mut app = vec![kind, u8::from(appendix == 2), 0];
        if kind >= 3 {
            app.extend(230_u16.to_be_bytes());
        } else {
            app.extend(if kind == 2 { [3, 1] } else { [6, 12] });
            if appendix == 0 {
                app.extend(if kind == 2 { 492_u16 } else { 13776 }.to_be_bytes());
                app.extend(if kind == 2 { 4_u16 } else { 200 }.to_be_bytes());
                app.push(if kind == 2 { 3 } else { 112 });
                if kind == 2 {
                    app.push(88);
                }
            } else {
                for count in if kind == 2 { vec![492_u16, 8, 8, 255, 24, 4, 8] } else { vec![13776_u16, 200, 112, 336, 112, 200] }
                {
                    app.extend(count.to_be_bytes());
                }
            }
        }
        sections.push((0x0501, appendix, app));
    }
    sections
}

fn download(sections: &[Section], wrapped: bool) -> Vec<u8> {
    let mut bytes = if wrapped { vec![0x76, 0x06] } else { Vec::new() };
    for (id, appendix, payload) in sections {
        bytes.extend(id.to_be_bytes());
        bytes.push(*appendix);
        bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
        bytes.extend(payload);
    }
    bytes
}

#[test]
fn all_card_types_and_generations_allow_either_or_both_common_efs_to_be_absent() {
    for kind in 1..=4 {
        for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
            for present in [vec![], vec![(2, 0, vec![0; 25])], vec![(5, 0, vec![0; 8])]] {
                let mut entries = sections(kind, gen1, gen2);
                entries.extend(present);
                for wrapped in [false, true] {
                    let bytes = download(&entries, wrapped);
                    let header = TachographHeader::from_data(&bytes).unwrap();
                    assert_eq!(header.data_type, TachographDataType::Card);
                    assert_eq!(header.card_in_vu_data, wrapped);
                    assert_eq!(
                        header.generation,
                        if gen2 { TachographDataGeneration::SecondGeneration } else { TachographDataGeneration::FirstGeneration }
                    );
                    let parsed = serde_json::to_value(parse_from_memory(&bytes).unwrap()).unwrap();
                    let response = &parsed["cardDataResponses"];
                    for card in if gen2 {
                        if gen1 { vec![&response["gen1"], &response["gen2"]] } else { vec![&response["gen2"]] }
                    } else {
                        vec![response]
                    } {
                        for (id, key, raw_key) in [(2, "cardIccIdentification", "ICC"), (5, "cardChipIdentification", "IC")] {
                            let present = entries.iter().any(|(file_id, _, _)| *file_id == id);
                            assert_eq!(!card[key].is_null(), present);
                            assert_eq!(card["dataFiles"].get(raw_key).is_some(), present);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn present_common_efs_are_still_validated_even_when_they_are_not_first() {
    for kind in 1..=4 {
        for (gen1, gen2) in [(true, false), (false, true), (true, true)] {
            for (id, size) in [(2, 25), (5, 8)] {
                for length in [0, size - 1, size + 1] {
                    let mut entries = sections(kind, gen1, gen2);
                    entries.push((id, 0, vec![0; length]));
                    assert!(parse_from_memory(&download(&entries, false)).is_err());
                }
            }
        }
    }
}

fn migration_sections(kind: u8) -> Vec<Section> {
    let mut entries = sections(kind, true, false);
    let mut app = entries.remove(0).2;
    app[3] = if kind == 2 { 3 } else { 6 };
    app[4] = if kind == 2 { 1 } else { 12 };
    let data = [
        (0x0501, app),
        (0x0520, vec![0; if kind == 2 { 211 } else { 143 }]),
        (0x0502, vec![0; 6 * usize::from(if kind == 2 { 3_u8 } else { 6 }) * 24]),
        (0x0503, vec![0; 2 * usize::from(if kind == 2 { 1_u8 } else { 12 }) * 24]),
        (0x0504, vec![0; 4 + if kind == 2 { 492 } else { 13776 }]),
        (0x0505, vec![0; 2 + if kind == 2 { 4 } else { 200 } * 31]),
        (0x0506, vec![0; 1 + if kind == 2 { 3 } else { 112 } * 10]),
        (0x0508, vec![0; 46]),
        (0x0522, vec![0; if kind == 2 { 2 * 5 } else { 56 * 5 }]),
    ];
    // Synthetic signatures exercise framing and retention; verification is tested separately.
    let mut entries = vec![(0xC100, 0, vec![0; 194]), (0xC108, 0, vec![0; 194])];
    for (id, payload) in data {
        entries.push((id, 0, payload));
        entries.push((id, 1, vec![0x55; 128]));
    }
    entries
}

#[test]
fn migration_downloads_retain_signed_application_data_and_certificates_in_file_and_memory_paths() {
    for kind in [1, 2] {
        let entries = migration_sections(kind);
        for wrapped in [false, true] {
            let bytes = download(&entries, wrapped);
            let path = TemporaryFile(std::env::temp_dir().join(format!(
                "esm-migration-{}-{}-{kind}-{wrapped}.ddd",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            )));
            std::fs::write(&path.0, &bytes).unwrap();
            let memory = serde_json::to_value(parse_from_memory(&bytes).unwrap()).unwrap();
            let file = serde_json::to_value(parse_from_file(path.0.to_str().unwrap()).unwrap()).unwrap();
            assert_eq!(memory, file);
            let card = &memory["cardDataResponses"];
            assert!(card["cardChipIdentification"].is_null());
            assert!(card["cardIccIdentification"].is_null());
            for (id, key) in [
                (0xC100, "CardCertificate"),
                (0xC108, "CACertificate"),
                (0x0501, "ApplicationIdentification"),
                (0x0520, "Identification"),
                (0x0502, "EventsData"),
                (0x0503, "FaultsData"),
                (0x0504, "DriverActivityData"),
                (0x0505, "VehiclesUsed"),
                (0x0506, "Places"),
                (0x0508, "ControlActivityData"),
                (0x0522, "SpecificConditions"),
            ] {
                let original = &entries.iter().find(|(file_id, appendix, _)| *file_id == id && *appendix == 0).unwrap().2;
                assert_eq!(card["dataFiles"][key]["data"], serde_json::to_value(original).unwrap());
                if id < 0xC000 {
                    assert_eq!(card["dataFiles"][key]["signature"], serde_json::json!(vec![0x55; 128]));
                } else {
                    assert_eq!(card["dataFiles"][key]["signature"], Value::Null);
                }
            }
        }
    }
}
