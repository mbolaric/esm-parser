use crate::Result;
use crate::gen2::VUOverview;
use crate::tacho::{EquipmentType, VerifyStatus, VuCertificateKind, VuVerifyItem, VuVerifyResult};
use crate::tachograph_gen2::verification::{self, GEN2_CERTIFICATE_SIZE, certificate_from_bytes, result_status};

/// Verifies a VU's own certificate chain (ERCA -> MSCA -> VU_Sign).
pub(crate) fn verify_certificate_chain(
    member_state_certificate_raw: &[u8],
    vu_certificate_raw: &[u8],
    erca_pk: &[u8; GEN2_CERTIFICATE_SIZE],
    validation_time: u32,
) -> Result<VuVerifyResult> {
    let (msca_certificate, link_certificate) = if member_state_certificate_raw.len() >= 2 * GEN2_CERTIFICATE_SIZE {
        let cert0 = certificate_from_bytes(&member_state_certificate_raw[..GEN2_CERTIFICATE_SIZE])?;
        let cert1 = certificate_from_bytes(&member_state_certificate_raw[GEN2_CERTIFICATE_SIZE..2 * GEN2_CERTIFICATE_SIZE])?;
        if cert0.certificate_holder_authorisation[verification::CHA_SIZE - 1] == EquipmentType::EuropeanRootCA as u8 {
            (cert1, Some(cert0))
        } else {
            (cert0, Some(cert1))
        }
    } else {
        (certificate_from_bytes(member_state_certificate_raw)?, None)
    };
    let vu_certificate = certificate_from_bytes(vu_certificate_raw)?;

    let (msca_verified, _link_verified, vu_verified) = verification::verify_vu_certificates_parsed_at(
        &msca_certificate,
        link_certificate.as_ref(),
        &vu_certificate,
        erca_pk,
        validation_time,
    )?;

    let result = vec![
        VuVerifyItem::certificate(
            VuCertificateKind::MemberStateCertificate,
            VerifyStatus::Valid,
            Some(msca_verified.end_of_validity),
        ),
        VuVerifyItem::certificate(VuCertificateKind::VuCertificate, VerifyStatus::Valid, Some(vu_verified.end_of_validity)),
    ];

    let status = result_status(result.iter().map(|item| item.status()));
    Ok(VuVerifyResult { status, result })
}

pub fn verify(vu_overview: &VUOverview, erca_pk: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<VuVerifyResult> {
    let validation_time = verification::current_unix_timestamp()?;
    verify_certificate_chain(&vu_overview.member_state_certificate_raw, &vu_overview.vu_certificate_raw, erca_pk, validation_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use crate::tacho::{EquipmentType, VerifyResultStatus};
    use crate::tachograph_gen2::card_verifiable_certificate::CHR_SIZE;
    use crate::tachograph_gen2::test_helpers::{certificate_for, signing_key};

    #[test]
    fn verifies_a_complete_vu_certificate_chain() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let vu_key = signing_key(3);
        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let vu_chr = [0x30; CHR_SIZE];
        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let vu_sign = certificate_for(&vu_key, &msca_key, msca_chr, vu_chr, EquipmentType::VehicleUnitSign as u8);

        let verified = verify_certificate_chain(&msca, &vu_sign, &root, 1_735_689_600).unwrap();

        assert!(matches!(verified.status, VerifyResultStatus::Valid));
        assert_eq!(verified.result.len(), 2);
        assert!(verified.result.iter().all(|item| matches!(item.status(), VerifyStatus::Valid)));
        assert!(
            verified.result.iter().any(|item| matches!(
                item,
                VuVerifyItem::Certificate { certificate: VuCertificateKind::MemberStateCertificate, .. }
            )) && verified
                .result
                .iter()
                .any(|item| matches!(item, VuVerifyItem::Certificate { certificate: VuCertificateKind::VuCertificate, .. }))
        );
    }

    #[test]
    fn rejects_a_vu_sign_certificate_with_the_wrong_equipment_type() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let card_key = signing_key(3);
        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let card_chr = [0x30; CHR_SIZE];
        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        // A DriverCardSign certificate must never be accepted as a VU_Sign certificate.
        let card_sign = certificate_for(&card_key, &msca_key, msca_chr, card_chr, EquipmentType::DriverCardSign as u8);

        let error = verify_certificate_chain(&msca, &card_sign, &root, 1_735_689_600).unwrap_err();

        assert!(
            matches!(error, Error::VerifyError(message) if message.contains("VU_Sign certificate has unsupported equipment type"))
        );
    }

    #[test]
    fn rejects_a_tampered_vu_sign_certificate() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let vu_key = signing_key(3);
        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let vu_chr = [0x30; CHR_SIZE];
        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let mut vu_sign = certificate_for(&vu_key, &msca_key, msca_chr, vu_chr, EquipmentType::VehicleUnitSign as u8);
        vu_sign[50] ^= 0x01;

        assert!(verify_certificate_chain(&msca, &vu_sign, &root, 1_735_689_600).is_err());
    }
}
