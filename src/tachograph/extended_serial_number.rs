use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{BCDString, Readable, Result};

/// Unique identification of an equipment. It can also be used as an
/// equipment Public Key Identifier.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
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

impl ExtendedSerialNumber {
    pub(crate) const MONTH_YEAR_LENGTH: usize = 2;

    pub(crate) fn read_optional<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Option<Self>> {
        let bytes = reader.read_bytes::<8>()?;
        // Appendix 1 uses all-FF for unknown or not-applicable equipment.
        if bytes == [0xFF; 8] {
            return Ok(None);
        }
        Self::read(&mut BinMemoryBuffer::from(bytes.as_slice())).map(Some)
    }
}

impl Readable for ExtendedSerialNumber {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<ExtendedSerialNumber> {
        let serial_number = reader.read_u32::<BigEndian>()?;
        let month_year = BCDString::decode(&reader.read_bytes::<{ ExtendedSerialNumber::MONTH_YEAR_LENGTH }>()?)?;
        let serial_type = reader.read_u8()?;
        let manufacturer_code = reader.read_u8()?;

        Ok(Self { serial_number, month_year, serial_type, manufacturer_code })
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_extended_serial_number_read() {
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x06, 0x24, 0x05, 0x0A];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let esn = ExtendedSerialNumber::read(&mut reader).unwrap();
        assert_eq!(esn.serial_number, 0x00010203);
        assert_eq!(esn.month_year, "0624");
        assert_eq!(esn.serial_type, 0x05);
        assert_eq!(esn.manufacturer_code, 0x0A);
    }
}
