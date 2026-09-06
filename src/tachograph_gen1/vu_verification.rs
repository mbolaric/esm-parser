use crate::Result;
use crate::gen1::VuOverview;
use crate::tacho::{VerifyStatus, VuCertificateKind, VuVerifyItem, VuVerifyResult};
use crate::tachograph_gen1::verification::{self, certificate_from_bytes, decrypt_certificate, result_status};

/// Verifies a VU's own certificate chain (ERCA -> MSCA -> VU).
pub(crate) fn verify_certificate_chain(
    member_state_certificate_raw: &[u8],
    vu_certificate_raw: &[u8],
    erca_pk: &[u8; 144],
) -> Result<VuVerifyResult> {
    let ec_pk_certificate = verification::ECPKCertificate::new(erca_pk)?;
    let msca_certificate = certificate_from_bytes(member_state_certificate_raw)?;
    let vu_certificate = certificate_from_bytes(vu_certificate_raw)?;

    let msca_decrypted =
        decrypt_certificate(&msca_certificate, ec_pk_certificate.holder_reference, &ec_pk_certificate.rsa_public_key)?;
    let vu_decrypted = decrypt_certificate(&vu_certificate, msca_decrypted.holder_reference, &msca_decrypted.rsa_public_key)?;

    let result = vec![
        VuVerifyItem {
            certificate: VuCertificateKind::MemberStateCertificate,
            status: VerifyStatus::Valid,
            end_of_validity: Some(msca_decrypted.end_of_validity.clone()),
        },
        VuVerifyItem {
            certificate: VuCertificateKind::VuCertificate,
            status: VerifyStatus::Valid,
            end_of_validity: Some(vu_decrypted.end_of_validity),
        },
    ];

    let status = result_status(result.iter().map(|item| &item.status));
    Ok(VuVerifyResult { status, result })
}

pub fn verify(vu_overview: &VuOverview, erca_pk: &[u8; 144]) -> Result<VuVerifyResult> {
    verify_certificate_chain(&vu_overview.member_state_certificate, &vu_overview.vu_certificate, erca_pk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn rejects_a_vu_certificate_of_the_wrong_length() {
        let error = verify_certificate_chain(&[0; 194], &[0; 193], &[0; 144]).unwrap_err();

        assert!(matches!(error, Error::VerifyError(message) if message.contains("Invalid signature length in Certificate")));
    }

    #[test]
    fn rejects_a_member_state_certificate_whose_authority_reference_does_not_match_the_erca() {
        let mut msca_bytes = [0u8; 194];
        msca_bytes[186..194].copy_from_slice(&[0x20; 8]);
        let mut erca_pk = [0u8; 144];
        erca_pk[..8].copy_from_slice(&[0x99; 8]);

        let error = verify_certificate_chain(&msca_bytes, &[0; 194], &erca_pk).unwrap_err();

        assert!(matches!(error, Error::VerifyError(message) if message.contains("issuer holder reference")));
    }
}
