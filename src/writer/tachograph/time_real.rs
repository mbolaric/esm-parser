use binary_data::{BigEndian, BinSeek, WriteBytes};
use time::OffsetDateTime;

use crate::tacho::TimeReal;
use crate::writer::{Validatable, Writable};

impl Validatable for TimeReal {
    /// Validates that the raw timestamp is either the unset sentinel (`0xFFFFFFFF`)
    /// or a valid Unix timestamp representable as an `OffsetDateTime`.
    fn validate(&self) -> crate::Result<()> {
        if self.data != TimeReal::TIME_REAL_NOT_SET && OffsetDateTime::from_unix_timestamp(self.data as i64).is_err() {
            return Err(crate::Error::ValidationError(format!("Invalid TimeReal timestamp: {}", self.data)));
        }
        Ok(())
    }
}

impl Writable for TimeReal {
    /// Writes a `TimeReal` to a binary stream, for creating or modifying DDD files.
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u32::<BigEndian>(self.data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::{BinMemoryBuffer, BinSeek};

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::Writable;

    #[test]
    fn test_write_time_real() {
        let timestamp: u32 = 1672531199;
        let time_real = TimeReal::new(timestamp);
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
}
