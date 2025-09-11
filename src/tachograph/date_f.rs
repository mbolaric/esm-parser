//! Represents a date structure specifically for tachograph data, where dates are BCD (Binary-Coded Decimal) encoded.
//! This format is common in DDD files from digital tachographs.

use std::fmt::Display;

use crate::{BCDString, Readable};

/// A date structure containing year, month, and day.
/// The values are stored as strings, as they are decoded from BCD format.
#[derive(Debug)]
pub struct Datef {
    pub year: String,
    pub month: String,
    pub day: String,
}

impl Display for Datef {
    /// Formats the date as a string in the format "YYYY-MM-DD".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Readable<Datef> for Datef {
    /// Reads a BCD-encoded date from a binary reader.
    /// The date is expected to be in a 4-byte format:
    /// - 2 bytes for the year
    /// - 1 byte for the month
    /// - 1 byte for the day
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Datef> {
        let year = BCDString::decode(&reader.read_into_vec(2)?)?;
        let month = BCDString::decode(&reader.read_into_vec(1)?)?;
        let day = BCDString::decode(&reader.read_into_vec(1)?)?;

        Ok(Self { year, month, day })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_data::BinMemoryBuffer;

    #[test]
    fn test_datef_read() {
        // BCD encoded date: 2023-09-11
        let data: Vec<u8> = vec![0x20, 0x23, 0x09, 0x11];
        let mut reader = BinMemoryBuffer::from(data);
        let datef = Datef::read(&mut reader).unwrap();

        assert_eq!(datef.year, "2023");
        assert_eq!(datef.month, "09");
        assert_eq!(datef.day, "11");
    }

    #[test]
    fn test_datef_display() {
        let datef = Datef { year: "2023".to_string(), month: "09".to_string(), day: "11".to_string() };

        assert_eq!(datef.to_string(), "2023-09-11");
    }
}
