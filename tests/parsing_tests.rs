use std::sync::OnceLock;

use esm_parser::{EsmParser, TachographData, gen1, tacho::EquipmentType};

#[derive(Debug)]
struct TestConfig {
    input_ddd_file_path: String,
}

static CONFIG: OnceLock<TestConfig> = OnceLock::new();

fn setup() -> &'static TestConfig {
    CONFIG.get_or_init(|| {
        let path = std::env::var("DDD_FILE_PATH").unwrap_or_else(|_| "examples/data/Card0001.DDD".to_string());
        TestConfig { input_ddd_file_path: path }
    })
}

#[test]
fn test_parse_gen1_card_file_successfully() {
    // --- Arrange ---
    let config = setup();
    let file_path = &config.input_ddd_file_path;

    // --- Act ---
    let result = EsmParser::parse(file_path);

    // --- Assert ---
    assert!(result.is_ok(), "Parsing failed with error: {:?}", result.err());

    let parser = result.unwrap();
    let data = parser.get_data().expect("Parser should have data after successful parse");

    // Check that the correct enum variant was parsed
    match data {
        TachographData::CardGen1(card_data) => match &card_data.card_data_responses {
            gen1::CardResponseParameterData::DriverCard(card) => {
                assert_eq!(card.application_identification.type_of_tachograph_card_id, EquipmentType::DriverCard);
                println!("Successfully parsed DriverCard Data.");
            }
            _ => panic!("Expected CardResponseParameterData::DriverCard, but found a different variant."),
        },
        _ => {
            panic!("Expected TachographData::CardGen1, but found a different variant.");
        }
    }
}

#[test]
fn test_parse_non_existent_file() {
    // --- Arrange ---
    let file_path = "path/to/non_existent_file.ddd";

    // --- Act ---
    let result = EsmParser::parse(file_path);

    // --- Assert ---
    assert!(result.is_err(), "Parsing should fail for a non-existent file.");

    let error = result.err().unwrap();
    // Check that the error is a file I/O error
    matches!(error, esm_parser::Error::File(_));
}
