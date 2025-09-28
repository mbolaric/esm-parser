use binary_data::BigEndian;
use serde::Serialize;

use crate::{BCDString, Readable};

const MONTH_YEAR_LENGTH: usize = 2;

/// Unique identification of an equipment. It can also be used as an
/// equipment Public Key Identifier.
#[derive(Debug, Serialize)]
pub struct ExtendedSerialNumber {
    #[serde(rename = "serialNumber")]
    pub serial_number: u32,
    #[serde(rename = "monthYear")]
    pub month_year: String,
    #[serde(rename = "type")]
    pub serial_type: u8,
    #[serde(rename = "manufacturerCode")]
    pub manufacturer_code: u8,
}

impl Readable<ExtendedSerialNumber> for ExtendedSerialNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ExtendedSerialNumber> {
        let serial_number = reader.read_u32::<BigEndian>()?;
        let month_year = BCDString::decode(&reader.read_bytes::<MONTH_YEAR_LENGTH>()?)?;
        let serial_type = reader.read_u8()?;
        let manufacturer_code = reader.read_u8()?;

        Ok(Self { serial_number, month_year, serial_type, manufacturer_code })
    }
}
