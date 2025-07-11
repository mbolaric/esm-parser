use crate::impl_enum_from_u16;

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum CertificateContentType {
    Unknown = 0x0,
    CertificateAuthorityReference = 0x42,
    ECCCertificate = 0x7F21,
    PublicKey = 0x7F49,
    ECCCertificateBody = 0x7F4E,
    CertificateHolderAuthorisation = 0x5F4C,
    CertificateHolderReference = 0x5F20,
    CertificateExpirationDate = 0x5F24,
    CertificateEffectiveDate = 0x5F25,
    CertificateProfileIdentifier = 0x5F29,
}

impl_enum_from_u16!(
    CertificateContentType {
        Unknown = 0x0,
        CertificateAuthorityReference = 0x42,
        ECCCertificate = 0x7F21,
        PublicKey = 0x7F49,
        ECCCertificateBody = 0x7F4E,
        CertificateHolderAuthorisation = 0x5F4C,
        CertificateHolderReference = 0x5F20,
        CertificateExpirationDate = 0x5F24,
        CertificateEffectiveDate = 0x5F25,
        CertificateProfileIdentifier = 0x5F29,
    }
);
