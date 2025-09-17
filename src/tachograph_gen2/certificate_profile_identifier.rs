use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, tacho::CertificateContentType};

#[derive(Debug, Serialize)]
pub struct CertificateProfileIdentifier {
    pub record_type: CertificateContentType,
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
