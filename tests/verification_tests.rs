use std::{env, fs, path::PathBuf};

use esm_parser::{
    TachographData,
    gen1::{self, CardResponseParameterData as CardResponseParameterDataGen1},
    gen2::{self, CardResponseParameterData as CardResponseParameterDataGen2, ParsedCard},
    parse_from_file,
    tacho::{CardFileID, CardFilesMap, DataFiles, VerifyResultStatus, VerifyStatus},
};

const COMBINED_DDD_ENV: &str = "ESM_PARSER_COMBINED_DDD";
const COMBINED_GEN1_ERCA_ENV: &str = "ESM_PARSER_COMBINED_GEN1_ERCA";
const COMBINED_GEN2_ERCA_ENV: &str = "ESM_PARSER_COMBINED_GEN2_ERCA";
const GEN1_DDD_ENV: &str = "ESM_PARSER_GEN1_DDD";
const GEN1_DDD_DIR_ENV: &str = "ESM_PARSER_GEN1_DDD_DIR";
const GEN1_ERCA_ENV: &str = "ESM_PARSER_GEN1_ERCA";

fn fixture_path(environment_variable: &str) -> PathBuf {
    env::var(environment_variable)
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("Set {environment_variable} to run the corresponding real card verification test."))
}

fn ddd_paths() -> Vec<PathBuf> {
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

fn card_data_files(data: &TachographData) -> &CardFilesMap {
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

fn combined_card_data_files(data: &TachographData) -> (&CardFilesMap, &CardFilesMap) {
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

fn is_non_signed_gen2_file(id: &CardFileID) -> bool {
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

fn tamper_first_signed_gen2_file(data_files: &mut CardFilesMap) -> CardFileID {
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

#[test]
#[ignore = "requires local Gen1 DDD and ERCA fixtures"]
fn test_verify_real_gen1_cards_and_rejects_tampering() {
    let erca_path = fixture_path(GEN1_ERCA_ENV);
    let erca: [u8; 144] = fs::read(erca_path)
        .expect("The configured Gen1 ERCA fixture must be readable.")
        .try_into()
        .expect("The configured Gen1 ERCA fixture must be exactly 144 bytes.");

    for ddd_path in ddd_paths() {
        let ddd_path_as_str = ddd_path.to_str().expect("The configured Gen1 DDD fixture path must be valid UTF-8.");
        let parsed = parse_from_file(ddd_path_as_str).expect("The configured Gen1 DDD fixture must parse.");
        let data_files = card_data_files(&parsed);

        let verified =
            gen1::verify(data_files, &erca).expect("The real Gen1 certificate chain and signed card data must verify.");
        assert!(matches!(verified.status, VerifyResultStatus::Valid), "Clean verification failed for {}.", ddd_path.display());
        assert!(
            verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)),
            "A checked file failed verification for {}.",
            ddd_path.display()
        );
        assert!(
            verified.result.iter().all(|item| item.card_file_id != CardFileID::IC && item.card_file_id != CardFileID::ICC),
            "A non-signed file was included in verification for {}.",
            ddd_path.display()
        );

        let mut tampered_data_files = data_files.clone();
        let card_download = tampered_data_files
            .get_mut(&CardFileID::CardDownload)
            .and_then(|data_file| data_file.data.as_mut())
            .expect("The configured Gen1 card fixture must include signed CardDownload data.");
        card_download[0] ^= 0x01;

        let tampered =
            gen1::verify(&tampered_data_files, &erca).expect("The certificate chain remains valid after data tampering.");
        assert!(
            matches!(tampered.status, VerifyResultStatus::PartiallyValid),
            "Tampering was not reflected in the aggregate result for {}.",
            ddd_path.display()
        );
        assert!(
            tampered
                .result
                .iter()
                .any(|item| { item.card_file_id == CardFileID::CardDownload && matches!(item.status, VerifyStatus::Invalid) }),
            "Tampering was not detected for {}.",
            ddd_path.display()
        );
    }
}

#[test]
#[ignore = "requires a local combined Driver or Workshop DDD plus Gen1 and Gen2 ERCA fixtures"]
fn test_verify_real_combined_card_and_rejects_gen2_tampering() {
    let ddd_path = fixture_path(COMBINED_DDD_ENV);
    let gen1_erca: [u8; 144] = fs::read(fixture_path(COMBINED_GEN1_ERCA_ENV))
        .expect("The configured combined-card Gen1 ERCA fixture must be readable.")
        .try_into()
        .expect("The configured combined-card Gen1 ERCA fixture must be exactly 144 bytes.");
    let gen2_erca: [u8; 205] = fs::read(fixture_path(COMBINED_GEN2_ERCA_ENV))
        .expect("The configured combined-card Gen2 ERCA fixture must be readable.")
        .try_into()
        .expect("The configured combined-card Gen2 ERCA fixture must be exactly 205 bytes.");

    let ddd_path_as_str = ddd_path.to_str().expect("The configured combined-card DDD fixture path must be valid UTF-8.");
    let parsed = parse_from_file(ddd_path_as_str).expect("The configured combined-card DDD fixture must parse.");
    let (gen1_data_files, gen2_data_files) = combined_card_data_files(&parsed);

    let gen1_verified =
        gen1::verify(gen1_data_files, &gen1_erca).expect("The combined card Gen1 certificate chain and signed data must verify.");
    assert!(matches!(gen1_verified.status, VerifyResultStatus::Valid));

    let gen2_verified =
        gen2::verify(gen2_data_files, &gen2_erca).expect("The combined card Gen2 certificate chain and signed data must verify.");
    assert!(matches!(gen2_verified.status, VerifyResultStatus::Valid));
    assert!(
        gen2_verified.result.iter().all(|item| item.card_file_id != CardFileID::CardDownload),
        "Gen2 CardDownload must not be reported as a signed elementary file."
    );

    let mut tampered_data_files = gen2_data_files.clone();
    let tampered_file_id = tamper_first_signed_gen2_file(&mut tampered_data_files);
    let tampered =
        gen2::verify(&tampered_data_files, &gen2_erca).expect("The Gen2 certificate chain remains valid after data tampering.");

    if gen2_verified.result.len() > 1 {
        assert!(matches!(tampered.status, VerifyResultStatus::PartiallyValid));
    } else {
        assert!(matches!(tampered.status, VerifyResultStatus::Invalid));
    }
    assert!(
        tampered.result.iter().any(|item| item.card_file_id == tampered_file_id && matches!(item.status, VerifyStatus::Invalid)),
        "Tampering was not detected for the selected Gen2 signed elementary file."
    );
}
