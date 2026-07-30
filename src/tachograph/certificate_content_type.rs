use serde::Serialize;

use crate::impl_enum_from_u16;

#[derive(Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u16)]
pub enum CertificateContentType {
    Unknown = 0x0,
    DomainParameters = 0x06,
    CertificateAuthorityReference = 0x42,
    PublicPoint = 0x86,
    ECCCertificate = 0x7F21,
    PublicKey = 0x7F49,
    ECCCertificateBody = 0x7F4E,
    CertificateHolderAuthorisation = 0x5F4C,
    CertificateHolderReference = 0x5F20,
    CertificateExpirationDate = 0x5F24,
    CertificateEffectiveDate = 0x5F25,
    CertificateProfileIdentifier = 0x5F29,
    CertificateSignature = 0x5F37,
}

impl_enum_from_u16!(
    CertificateContentType {
        Unknown = 0x0,
        DomainParameters = 0x06,
        CertificateAuthorityReference = 0x42,
        PublicPoint = 0x86,
        ECCCertificate = 0x7F21,
        PublicKey = 0x7F49,
        ECCCertificateBody = 0x7F4E,
        CertificateHolderAuthorisation = 0x5F4C,
        CertificateHolderReference = 0x5F20,
        CertificateExpirationDate = 0x5F24,
        CertificateEffectiveDate = 0x5F25,
        CertificateProfileIdentifier = 0x5F29,
        CertificateSignature = 0x5F37,
    }
);
