use crate::Result;
use crate::gen2::VUOverview;
use crate::tacho::{VerifyStatus, VuCertificateKind, VuVerifyItem, VuVerifyResult};
use crate::tachograph_gen2::verification::{
    self, GEN2_CERTIFICATE_SIZE, SigningRole, certificate_from_bytes, result_status, verify_ca_certificate_at,
    verify_leaf_certificate_at,
};

/// Verifies a VU's own certificate chain (ERCA -> MSCA -> VU_Sign).
pub(crate) fn verify_certificate_chain(
    member_state_certificate_raw: &[u8],
    vu_certificate_raw: &[u8],
    erca_pk: &[u8; GEN2_CERTIFICATE_SIZE],
    validation_time: u32,
) -> Result<VuVerifyResult> {
    let erca_certificate = verification::ERCACertificate::new_at(erca_pk, validation_time)?;
    let msca_certificate = certificate_from_bytes(member_state_certificate_raw)?;
    let vu_certificate = certificate_from_bytes(vu_certificate_raw)?;

    let msca_verified = verify_ca_certificate_at(&msca_certificate, &erca_certificate, validation_time)?;
    let vu_verified = verify_leaf_certificate_at(&vu_certificate, &msca_verified, validation_time, SigningRole::VuSign)?;

    let result = vec![
        VuVerifyItem {
            certificate: VuCertificateKind::MemberStateCertificate,
            status: VerifyStatus::Valid,
            end_of_validity: Some(msca_verified.end_of_validity.clone()),
        },
        VuVerifyItem {
            certificate: VuCertificateKind::VuCertificate,
            status: VerifyStatus::Valid,
            end_of_validity: Some(vu_verified.end_of_validity),
        },
    ];

    let status = result_status(result.iter().map(|item| &item.status));
    Ok(VuVerifyResult { status, result })
}

pub fn verify(vu_overview: &VUOverview, erca_pk: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<VuVerifyResult> {
    let validation_time = verification::current_unix_timestamp()?;
    verify_certificate_chain(&vu_overview.member_state_certificate_raw, &vu_overview.vu_certificate_raw, erca_pk, validation_time)
}

#[cfg(test)]
mod tests {
    use bp256::BrainpoolP256r1;
    use bp256::r1::ecdsa::Signature;
    use ecdsa::SigningKey;
    use sha2::{Digest, Sha256};
    use signature::hazmat::PrehashSigner;

    use super::*;
    use crate::Error;
    use crate::tacho::{EquipmentType, VerifyResultStatus};

    const CAR_SIZE: usize = 8;
    const CHR_SIZE: usize = 8;
    const CPI_VERSION_1: u8 = 0x00;
    const SEC1_UNCOMPRESSED_P256_POINT_SIZE: usize = 65;

    fn signing_key(last_byte: u8) -> SigningKey<BrainpoolP256r1> {
        let mut scalar = [0u8; 32];
        scalar[31] = last_byte;
        SigningKey::from_slice(&scalar).unwrap()
    }

    fn sign(signer: &SigningKey<BrainpoolP256r1>, payload: &[u8]) -> Vec<u8> {
        let signature: Signature = signer.sign_prehash(&Sha256::digest(payload)).unwrap();
        signature.to_bytes().as_slice().to_vec()
    }

    fn certificate_for(
        holder_key: &SigningKey<BrainpoolP256r1>,
        issuer_key: &SigningKey<BrainpoolP256r1>,
        car: [u8; CAR_SIZE],
        chr: [u8; CHR_SIZE],
        equipment_type: u8,
    ) -> [u8; GEN2_CERTIFICATE_SIZE] {
        let public_point = holder_key.verifying_key().to_sec1_point(false);
        let public_point = public_point.as_bytes();
        assert_eq!(public_point.len(), SEC1_UNCOMPRESSED_P256_POINT_SIZE);

        let mut body = vec![0x7F, 0x4E, 0x81, 0x82];
        body.extend_from_slice(&[0x5F, 0x29, 0x01, CPI_VERSION_1]);
        body.extend_from_slice(&[0x42, 0x08]);
        body.extend_from_slice(&car);
        body.extend_from_slice(&[0x5F, 0x4C, 0x07, 0xFF, b'S', b'M', b'R', b'D', b'T', equipment_type]);
        body.extend_from_slice(&[0x7F, 0x49, 0x4E, 0x06, 0x09]);
        body.extend_from_slice(&[0x2B, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x07]);
        body.extend_from_slice(&[0x86, 0x41]);
        body.extend_from_slice(public_point);
        body.extend_from_slice(&[0x5F, 0x20, 0x08]);
        body.extend_from_slice(&chr);
        body.extend_from_slice(&[0x5F, 0x25, 0x04, 0x00, 0x00, 0x00, 0x00]);
        body.extend_from_slice(&[0x5F, 0x24, 0x04, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(body.len(), 134);

        let signature = sign(issuer_key, &body);
        let mut certificate = vec![0x7F, 0x21, 0x81, 0xC9];
        certificate.extend_from_slice(&body);
        certificate.extend_from_slice(&[0x5F, 0x37, 0x40]);
        certificate.extend_from_slice(&signature);
        certificate.try_into().unwrap()
    }

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
        assert!(verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)));
        assert!(
            verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::MemberStateCertificate))
                && verified.result.iter().any(|item| matches!(item.certificate, VuCertificateKind::VuCertificate))
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
