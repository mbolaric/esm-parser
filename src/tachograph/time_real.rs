use binary_data::{BigEndian, BinSeek, WriteBytes};
use chrono::Utc;

use crate::{Readable, Writable};

/// Represents a real-time timestamp from a tachograph DDD file, stored as a u32 Unix timestamp.
#[derive(Debug)]
pub struct TimeReal {
    /// The raw Unix timestamp value from the DDD file.
    pub data: u32,
    /// The `chrono::DateTime<Utc>` representation of the timestamp.
    date_time: Option<chrono::DateTime<Utc>>,
}

impl TimeReal {
    /// Returns the date part of the timestamp as a string in "YYYY-MM-DD" format.
    /// This is useful for extracting the date of an event from the tachograph data.
    pub fn get_date_str(&self) -> String {
        self.date_time.map_or("".to_owned(), |data| data.format("%Y-%m-%d").to_string())
    }

    /// Returns the date and time part of the timestamp as a string in "YYYY-MM-DD HH:MM:SS" format.
    /// This provides a full timestamp for an event from the tachograph data.
    pub fn get_date_time_str(&self) -> String {
        self.date_time.map_or("".to_owned(), |data| data.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    /// Returns the time part of the timestamp as a string in "HH:MM:SS" format.
    /// This is useful for extracting the time of an event from the tachograph data.
    pub fn get_time_str(&self) -> String {
        self.date_time.map_or("".to_owned(), |data| data.format("%H:%M:%S").to_string())
    }

    /// Returns the raw u32 timestamp value.
    pub fn get_data(&self) -> u32 {
        self.data
    }

    /// Checks if the timestamp has a non-zero value.
    /// In the context of DDD files, a zero value often means the timestamp is not set.
    pub fn has_data(&self) -> bool {
        self.data != 0
    }
}

impl Readable<TimeReal> for TimeReal {
    /// Reads a `TimeReal` from a binary stream of a DDD file.
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<TimeReal> {
        let data = reader.read_u32::<BigEndian>()?;
        let date_time = chrono::DateTime::from_timestamp(data as i64, 0);
        Ok(Self { data, date_time })
    }
}

impl Writable for TimeReal {
    /// Writes a `TimeReal` to a binary stream, for creating or modifying DDD files.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u32::<BigEndian>(self.data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_data::BinMemoryBuffer;

    #[test]
    fn test_read_time_real() {
        let timestamp: u32 = 1672531199; // 2022-12-31 23:59:59
        let mut reader = BinMemoryBuffer::from(timestamp.to_be_bytes().to_vec());
        let time_real = TimeReal::read(&mut reader).unwrap();
        assert_eq!(time_real.get_data(), timestamp);
    }

    #[test]
    fn test_write_time_real() {
        let timestamp: u32 = 1672531199;
        let time_real = TimeReal { data: timestamp, date_time: chrono::DateTime::from_timestamp(timestamp as i64, 0) };
        let mut writer = BinMemoryBuffer::new();
        time_real.write(&mut writer).unwrap();
        let _ = writer.seek(0);
        assert_eq!(writer.remaining_slice(), &timestamp.to_be_bytes());
    }

    #[test]
    fn test_read_and_write_time_real() {
        let timestamp: u32 = 1672531199; // 2022-12-31 23:59:59
        let mut reader = BinMemoryBuffer::from(timestamp.to_be_bytes().to_vec());
        let time_real = TimeReal::read(&mut reader).unwrap();
        assert_eq!(time_real.get_data(), timestamp);

        let mut writer = BinMemoryBuffer::new();
        time_real.write(&mut writer).unwrap();
        let _ = writer.seek(0);
        assert_eq!(writer.remaining_slice(), &timestamp.to_be_bytes());
    }

    #[test]
    fn test_time_real_getters() {
        let timestamp: u32 = 1672531199; // 2022-12-31 23:59:59 UTC
        let mut reader = BinMemoryBuffer::from(timestamp.to_be_bytes().to_vec());
        let time_real = TimeReal::read(&mut reader).unwrap();

        assert_eq!(time_real.get_date_str(), "2022-12-31");
        assert_eq!(time_real.get_date_time_str(), "2022-12-31 23:59:59");
        assert_eq!(time_real.get_time_str(), "23:59:59");
        assert_eq!(time_real.get_data(), timestamp);
        assert!(time_real.has_data());
    }

    #[test]
    fn test_time_real_zero_timestamp() {
        let timestamp: u32 = 0;
        let mut reader = BinMemoryBuffer::from(timestamp.to_be_bytes().to_vec());
        let time_real = TimeReal::read(&mut reader).unwrap();

        assert_eq!(time_real.get_date_str(), "1970-01-01");
        assert_eq!(time_real.get_date_time_str(), "1970-01-01 00:00:00");
        assert_eq!(time_real.get_time_str(), "00:00:00");
        assert_eq!(time_real.get_data(), 0);
        assert!(!time_real.has_data());
    }
}
