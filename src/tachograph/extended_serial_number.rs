use binary_data::BigEndian;
use serde::Serialize;

use crate::{BCDString, Readable, Validatable, Writable, validate_bcd};

const MONTH_YEAR_LENGTH: usize = 2;

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

impl Validatable for ExtendedSerialNumber {
    fn validate(&self) -> crate::Result<()> {
        validate_bcd::<MONTH_YEAR_LENGTH>("ExtendedSerialNumber.month_year", &self.month_year)?;
        if self.month_year != "0000" {
            let month: u32 = self.month_year[0..2]
                .parse()
                .map_err(|_| crate::Error::ValidationError("Invalid month in ExtendedSerialNumber".into()))?;
            if !(1..=12).contains(&month) {
                return Err(crate::Error::ValidationError(format!(
                    "ExtendedSerialNumber month {month:02} out of range 01..=12 in month_year {}",
                    self.month_year
                )));
            }
        }
        Ok(())
    }
}

impl Readable for ExtendedSerialNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ExtendedSerialNumber> {
        let serial_number = reader.read_u32::<BigEndian>()?;
        let month_year = BCDString::decode(&reader.read_bytes::<MONTH_YEAR_LENGTH>()?)?;
        let serial_type = reader.read_u8()?;
        let manufacturer_code = reader.read_u8()?;

        Ok(Self { serial_number, month_year, serial_type, manufacturer_code })
    }
}

impl Writable for ExtendedSerialNumber {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u32::<BigEndian>(self.serial_number)?;
        let month_year_bytes = validate_bcd::<MONTH_YEAR_LENGTH>("ExtendedSerialNumber.month_year", &self.month_year)?;
        writer.write_all(&month_year_bytes)?;
        writer.write_u8(self.serial_type)?;
        writer.write_u8(self.manufacturer_code)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_extended_serial_number_roundtrip() {
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x06, 0x24, 0x05, 0x0A];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let esn = ExtendedSerialNumber::read(&mut reader).unwrap();
        assert_eq!(esn.serial_number, 0x00010203);
        assert_eq!(esn.month_year, "0624");
        assert_eq!(esn.serial_type, 0x05);
        assert_eq!(esn.manufacturer_code, 0x0A);

        let mut writer = BinMemoryBuffer::new();
        esn.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_extended_serial_number_validation_and_zero_byte_emission() {
        // Invalid month 13
        let invalid_month = ExtendedSerialNumber {
            serial_number: 123,
            month_year: "1324".into(),
            serial_type: 1,
            manufacturer_code: 2,
        };
        assert!(invalid_month.validate().is_err());
        let mut writer = BinMemoryBuffer::new();
        assert!(invalid_month.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        // Sentinel "0000" is valid
        let sentinel = ExtendedSerialNumber {
            serial_number: 0,
            month_year: "0000".into(),
            serial_type: 0,
            manufacturer_code: 0,
        };
        assert!(sentinel.validate().is_ok());
    }
}
