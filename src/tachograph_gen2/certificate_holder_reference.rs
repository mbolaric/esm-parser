use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, tacho::CertificateContentType};

/// The Certificate Holder Reference is an identifier for the
/// public key provided in the certificate. It shall be used to
/// reference this public key in other certificates.
#[derive(Debug, Serialize)]
pub struct CertificateHolderReference {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    pub data: Vec<u8>,
}

impl Readable<CertificateHolderReference> for CertificateHolderReference {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CertificateHolderReference> {
        let record_type: CertificateContentType = reader.read_u16::<BigEndian>()?.into();
        let record_size = reader.read_u8()? as u16;
        let data = reader.read_into_vec(record_size as u32)?;

        Ok(Self { record_type, record_size, data })
    }
}
