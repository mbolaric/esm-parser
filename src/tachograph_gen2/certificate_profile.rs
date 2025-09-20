use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{
        CertificateAuthorityReference, CertificateDate, CertificateHolderReference, CertificateParams,
        CertificateProfileIdentifier, EccCertificate,
    },
    tacho::{CertificateHolderAuthorisation, PublicKey, PublicKeyParams},
};

#[derive(Debug)]
pub struct CertificateProfileParams {
    pub size: u16,
}

impl CertificateProfileParams {
    pub fn new(size: u16) -> Self {
        Self { size }
    }
}

#[derive(Debug, Serialize)]
pub struct CertificateProfile {
    #[serde(rename = "eccCertificate")]
    pub ecc_certificate: EccCertificate,
    #[serde(rename = "eccCertificateBody")]
    pub ecc_certificate_body: EccCertificate,
    #[serde(rename = "certificateProfileIdentifier")]
    pub certificate_profile_identifier: CertificateProfileIdentifier,
    #[serde(rename = "certificateAuthorityReference")]
    pub certificate_authority_reference: CertificateAuthorityReference,
    #[serde(rename = "certificateHolderAuthorisation")]
    pub certificate_holder_authorisation: CertificateHolderAuthorisation,
    #[serde(rename = "publicKey")]
    pub public_key: Option<PublicKey>,
    #[serde(rename = "certificateHolderReference")]
    pub certificate_holder_reference: CertificateHolderReference,
    #[serde(rename = "certificateEffectiveDate")]
    pub certificate_effective_date: CertificateDate,
    #[serde(rename = "certificateExpirationDate")]
    pub certificate_expiration_date: CertificateDate,
}

impl ReadableWithParams<CertificateProfile> for CertificateProfile {
    type P = CertificateProfileParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CertificateProfile> {
        let pk_length = params.size;
        let ecc_certificate = EccCertificate::read(reader, &CertificateParams::new(Some(2)))?;
        let ecc_certificate_body = EccCertificate::read(reader, &CertificateParams::new(Some(2)))?;
        let certificate_profile_identifier = CertificateProfileIdentifier::read(reader)?;
        let certificate_authority_reference = CertificateAuthorityReference::read(reader)?;
        let certificate_holder_authorisation = CertificateHolderAuthorisation::read(reader)?;
        let public_key = if pk_length == 204 {
            let params = PublicKeyParams::new(Some(78));
            Some(PublicKey::read(reader, &params)?)
        } else if pk_length == 205 {
            let params = PublicKeyParams::new(Some(79));
            Some(PublicKey::read(reader, &params)?)
        } else {
            None
        };
        let certificate_holder_reference = CertificateHolderReference::read(reader)?;
        let certificate_effective_date = CertificateDate::read(reader)?;
        let certificate_expiration_date = CertificateDate::read(reader)?;

        Ok(Self {
            ecc_certificate,
            ecc_certificate_body,
            certificate_profile_identifier,
            certificate_authority_reference,
            certificate_holder_authorisation,
            public_key,
            certificate_holder_reference,
            certificate_effective_date,
            certificate_expiration_date,
        })
    }
}
