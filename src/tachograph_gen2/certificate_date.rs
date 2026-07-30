use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::{CertificateContentType, TimeReal};
use crate::{Readable, Result};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2CertificateDate"))]
pub struct CertificateDate {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordSize")]
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

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn serializes_record_size_under_its_own_key() {
        let mut reader = BinMemoryBuffer::from(vec![0x00, 0x01, 0x07, 0x00, 0x00, 0x00, 0x00]);
        let value = CertificateDate::read(&mut reader).expect("certificate date should parse");
        let serialized = serde_json::to_value(value).expect("certificate date should serialize");

        assert_eq!(serialized["recordSize"], 7);
        assert!(serialized.get("recordType").is_some());
    }
}
