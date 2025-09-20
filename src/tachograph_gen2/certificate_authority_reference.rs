use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    tacho::{CertificateContentType, CertificationAuthorityKid},
};

#[derive(Debug, Serialize)]
pub struct CertificateAuthorityReference {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "certificationAuthorityKid")]
    pub certification_authority_kid: CertificationAuthorityKid,
}

impl Readable<CertificateAuthorityReference> for CertificateAuthorityReference {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CertificateAuthorityReference> {
        let record_type: CertificateContentType = (reader.read_u8()? as u16).into();
        let record_size = reader.read_u8()? as u16;
        let certification_authority_kid = CertificationAuthorityKid::read(reader)?;
        Ok(Self { record_type, record_size, certification_authority_kid })
    }
}
