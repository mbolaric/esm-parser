//! Helpers shared by the integration tests.

use std::path::PathBuf;
use std::{env, fs};

use esm_parser::gen1::CardResponseParameterData as CardResponseParameterDataGen1;
use esm_parser::gen2::{self, CardResponseParameterData as CardResponseParameterDataGen2, ParsedCard};
use esm_parser::tacho::{CardFileID, CardFilesMap, DataFiles, VUData as _};
use esm_parser::{TachographData, gen1};

pub const COMBINED_DDD_ENV: &str = "ESM_PARSER_COMBINED_DDD";
pub const COMBINED_GEN1_ERCA_ENV: &str = "ESM_PARSER_COMBINED_GEN1_ERCA";
pub const COMBINED_GEN2_ERCA_ENV: &str = "ESM_PARSER_COMBINED_GEN2_ERCA";
pub const GEN1_DDD_ENV: &str = "ESM_PARSER_GEN1_DDD";
pub const GEN1_DDD_DIR_ENV: &str = "ESM_PARSER_GEN1_DDD_DIR";
pub const GEN1_ERCA_ENV: &str = "ESM_PARSER_GEN1_ERCA";
pub const GEN1_VU_DDD_ENV: &str = "ESM_PARSER_GEN1_VU_DDD";
pub const GEN1_VU_ERCA_ENV: &str = "ESM_PARSER_GEN1_VU_ERCA";
pub const GEN2_VU_DDD_ENV: &str = "ESM_PARSER_GEN2_VU_DDD";
pub const GEN2_VU_ERCA_ENV: &str = "ESM_PARSER_GEN2_VU_ERCA";

pub fn fixture_path(environment_variable: &str) -> PathBuf {
    env::var(environment_variable)
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Set {environment_variable} to run the corresponding real card verification test."))
}

pub fn ddd_paths() -> Vec<PathBuf> {
    let Ok(directory) = env::var(GEN1_DDD_DIR_ENV) else {
        return vec![fixture_path(GEN1_DDD_ENV)];
    };

    let mut paths = fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("Could not read {directory}: {error}"))
        .map(|entry| entry.expect("Could not read a Gen1 DDD fixture directory entry.").path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("ddd")))
        .collect::<Vec<_>>();
    paths.sort();
    assert!(!paths.is_empty(), "No .ddd fixtures found in {directory}.");
    paths
}

pub fn card_data_files(data: &TachographData) -> &CardFilesMap {
    let TachographData::CardGen1(card_data) = data else {
        panic!("The configured fixture is not a Gen1 card download.");
    };
    let card: &dyn DataFiles = match &card_data.card_data_responses {
        CardResponseParameterDataGen1::DriverCard(card) => card.as_ref(),
        CardResponseParameterDataGen1::CompanyCard(card) => card.as_ref(),
        CardResponseParameterDataGen1::WorkshopCard(card) => card.as_ref(),
        CardResponseParameterDataGen1::ControlCard(card) => card.as_ref(),
        CardResponseParameterDataGen1::Unsupported => panic!("The configured fixture has an unsupported card type."),
    };
    card.get_data_files()
}

pub fn combined_card_data_files(data: &TachographData) -> (&CardFilesMap, &CardFilesMap) {
    let TachographData::CardGen2(card_data) = data else {
        panic!("The configured fixture is not a Gen2 combined card download.");
    };

    match &card_data.card_data_responses {
        CardResponseParameterDataGen2::DriverCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
            (gen1_card.get_data_files(), gen2_card.get_data_files())
        }
        CardResponseParameterDataGen2::WorkshopCard(ParsedCard::Combined(gen1_card, gen2_card)) => {
            (gen1_card.get_data_files(), gen2_card.get_data_files())
        }
        CardResponseParameterDataGen2::CompanyCard(ParsedCard::Combined(_, _))
        | CardResponseParameterDataGen2::ControlCard(ParsedCard::Combined(_, _)) => {
            panic!("A Company or Control combined card has no Gen2 Card_Sign verification path.");
        }
        _ => panic!("The configured fixture is not a combined Driver or Workshop card."),
    }
}

pub fn is_non_signed_gen2_file(id: &CardFileID) -> bool {
    matches!(
        id,
        CardFileID::IC
            | CardFileID::ICC
            | CardFileID::CACertificate
            | CardFileID::CardCertificate
            | CardFileID::CardSignCertificate
            | CardFileID::LinkCertificate
            | CardFileID::CardDownload
    )
}

pub fn tamper_first_signed_gen2_file(data_files: &mut CardFilesMap) -> CardFileID {
    for (id, data_file) in data_files.iter_mut() {
        if is_non_signed_gen2_file(id) || data_file.signature.is_none() {
            continue;
        }
        if let Some(first_byte) = data_file.data.as_mut().and_then(|data| data.first_mut()) {
            *first_byte ^= 0x01;
            return id.clone();
        }
    }

    panic!("The configured Gen2 card fixture must include signed application data.");
}

pub fn gen1_vu_overview(data: &TachographData) -> &gen1::VuOverview {
    let TachographData::VUGen1(vu_data) = data else {
        panic!("The configured fixture is not a Gen1 VU download.");
    };
    vu_data
        .get_data()
        .iter()
        .find_map(|item| match &item.data {
            gen1::VUTransferResponseParameterData::Control(overview) => Some(overview),
            _ => None,
        })
        .expect("The configured Gen1 VU fixture must include an Overview (TREP 01) block.")
}

pub fn gen2_vu_overview(data: &TachographData) -> &gen2::VUOverview {
    let TachographData::VUGen2(vu_data) = data else {
        panic!("The configured fixture is not a Gen2 VU download.");
    };
    vu_data
        .get_data()
        .iter()
        .find_map(|item| match &item.data {
            gen2::VUTransferResponseParameterData::Control(overview) => Some(overview),
            _ => None,
        })
        .expect("The configured Gen2 VU fixture must include an Overview (TREP 21/31) block.")
}
