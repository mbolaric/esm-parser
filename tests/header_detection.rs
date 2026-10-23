#[path = "common/card.rs"]
mod card;

use std::io::Write;
use std::path::PathBuf;

use card::section;
use esm_parser::tacho::{TachographDataGeneration, TachographHeader};
use esm_parser::{TachographData, parse_from_file, parse_from_memory};

struct TemporaryFile(PathBuf);

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn card(generation: u8, wrapped: bool) -> Vec<u8> {
    let application = if generation == 0 {
        vec![1, 0, 0, 6, 12, 0x15, 0xA8, 0, 84, 84]
    } else {
        vec![1, 1, 0, 6, 12, 0x15, 0xA8, 0, 84, 0, 84, 0, 252, 0, 56, 0, 84]
    };
    let mut bytes = if wrapped { vec![0x76, 0x06] } else { Vec::new() };
    for (id, appendix, payload) in [(2_u16, 0, vec![0; 25]), (5, 0, vec![0; 8]), (0x0501, generation * 2, application)] {
        section(&mut bytes, id, appendix, &payload);
    }
    bytes
}

#[test]
fn header_and_both_parsing_entry_points_agree_on_small_card_generations() {
    for generation in [0, 1] {
        for wrapped in [false, true] {
            let bytes = card(generation, wrapped);
            let header = TachographHeader::from_data(&bytes).unwrap();
            let expected = if generation == 0 {
                TachographDataGeneration::FirstGeneration
            } else {
                TachographDataGeneration::SecondGeneration
            };
            assert_eq!(header.generation, expected);
            let path = TemporaryFile(std::env::temp_dir().join(format!(
                "esm-header-{}-{}-{generation}-{wrapped}.ddd",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            )));
            let mut output = std::fs::OpenOptions::new().write(true).create_new(true).open(&path.0).unwrap();
            output.write_all(&bytes).unwrap();
            drop(output);
            let memory = parse_from_memory(&bytes).unwrap();
            let file = parse_from_file(path.0.to_str().unwrap()).unwrap();
            assert_eq!(serde_json::to_value(&memory).unwrap(), serde_json::to_value(file).unwrap());
            match memory {
                TachographData::CardGen1(card) => assert_eq!(card.header.generation, expected),
                TachographData::CardGen2(card) => assert_eq!(card.header.generation, expected),
                _ => panic!("expected card"),
            }
        }
    }
}
