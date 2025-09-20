use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    tacho::{CertificateContentType, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct CertificateDate {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordType")]
    pub record_size: u16,
    pub date: TimeReal,
}

impl Readable<CertificateDate> for CertificateDate {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CertificateDate> {
        let record_type: CertificateContentType = reader.read_u16::<BigEndian>()?.into();
        let record_size = reader.read_u8()? as u16;
        let date = TimeReal::read(reader)?;

        Ok(Self { record_type, record_size, date })
    }
}
