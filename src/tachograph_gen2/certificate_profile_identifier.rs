use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::CertificateContentType;
use crate::{Readable, Result};

/// Certificates shall use a Certificate Profile Identifier to
/// indicate the certificate profile used. Version 1, shall be identified by a value of ‘00’.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2CertificateProfileIdentifier"))]
pub struct CertificateProfileIdentifier {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    pub data: Vec<u8>,
}

impl Readable<CertificateProfileIdentifier> for CertificateProfileIdentifier {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CertificateProfileIdentifier> {
        let record_type: CertificateContentType = reader.read_u16::<BigEndian>()?.into();
        let record_size = reader.read_u8()? as u16;
        let data = reader.read_into_vec(record_size as u32)?;

        Ok(Self { record_type, record_size, data })
    }
}
