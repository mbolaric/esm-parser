mod common;

use std::fs;

use common::{
    COMBINED_DDD_ENV, COMBINED_GEN1_ERCA_ENV, COMBINED_GEN2_ERCA_ENV, GEN1_ERCA_ENV, GEN1_VU_DDD_ENV, GEN1_VU_ERCA_ENV,
    GEN2_VU_DDD_ENV, GEN2_VU_ERCA_ENV, card_data_files, combined_card_data_files, ddd_paths, fixture_path, gen1_vu_overview,
    gen2_vu_overview, tamper_first_signed_gen2_file,
};
use esm_parser::tacho::{CardFileID, VerifyResultStatus, VerifyStatus, VuCertificateKind};
use esm_parser::{VuOverview, gen1, gen2, parse_from_file, verify_vu};

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

#[test]
#[ignore = "requires a local Gen1 VU download DDD and Gen1 ERCA fixture"]
fn test_verify_real_gen1_vu_certificate_chain() {
    let ddd_path = fixture_path(GEN1_VU_DDD_ENV);
    let erca: [u8; 144] = fs::read(fixture_path(GEN1_VU_ERCA_ENV))
        .expect("The configured Gen1 VU ERCA fixture must be readable.")
        .try_into()
        .expect("The configured Gen1 VU ERCA fixture must be exactly 144 bytes.");

    let ddd_path_as_str = ddd_path.to_str().expect("The configured Gen1 VU DDD fixture path must be valid UTF-8.");
    let parsed = parse_from_file(ddd_path_as_str).expect("The configured Gen1 VU DDD fixture must parse.");
    let overview = gen1_vu_overview(&parsed);

    let verified = verify_vu(&VuOverview::Gen1(overview), &erca).expect("The real Gen1 VU certificate chain must verify.");
    assert!(matches!(verified.status, VerifyResultStatus::Valid));
    assert!(verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)));
    assert!(verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::MemberStateCertificate)));
    assert!(verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::VuCertificate)));
}

#[test]
#[ignore = "requires a local Gen2 VU download DDD and Gen2 ERCA fixture"]
fn test_verify_real_gen2_vu_certificate_chain() {
    let ddd_path = fixture_path(GEN2_VU_DDD_ENV);
    let erca: [u8; 205] = fs::read(fixture_path(GEN2_VU_ERCA_ENV))
        .expect("The configured Gen2 VU ERCA fixture must be readable.")
        .try_into()
        .expect("The configured Gen2 VU ERCA fixture must be exactly 205 bytes.");

    let ddd_path_as_str = ddd_path.to_str().expect("The configured Gen2 VU DDD fixture path must be valid UTF-8.");
    let parsed = parse_from_file(ddd_path_as_str).expect("The configured Gen2 VU DDD fixture must parse.");
    let overview = gen2_vu_overview(&parsed);

    let verified = verify_vu(&VuOverview::Gen2(overview), &erca).expect("The real Gen2 VU certificate chain must verify.");
    assert!(matches!(verified.status, VerifyResultStatus::Valid));
    assert!(verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)));
    assert!(verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::MemberStateCertificate)));
    assert!(verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::VuCertificate)));
}
