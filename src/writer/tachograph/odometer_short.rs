use crate::tacho::OdometerShort;
use crate::writer::{Validatable, Writable};

impl Validatable for OdometerShort {
    /// Validates the odometer value according to EU tachograph regulations (Annex 1B/1C):
    /// - Operating values must not exceed 9,999,999 km.
    /// - `None` represents the sentinel value `0xFFFFFF` ("value not available") and is valid.
    /// - Values above 9,999,999 (including raw `0xFFFFFF` passed in `Some`) are rejected with `RecordOutOfRange`.
    fn validate(&self) -> crate::Result<()> {
        if let Some(val) = self.data
            && val > 9_999_999
        {
            return Err(crate::Error::RecordOutOfRange(format!(
                "OdometerShort value {val} exceeds operating limit 9,999,999 km"
            )));
        }
        Ok(())
    }
}

impl Writable for OdometerShort {
    /// Writes the 3-byte big-endian binary representation of the odometer value.
    ///
    /// If `data` is `None`, writes the sentinel `0xFFFFFF`.
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        let val = self.data.unwrap_or(0xFFFFFF);
        let bytes = [(val >> 16) as u8, (val >> 8) as u8, val as u8];
        writer.write_all(&bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    #[test]
    fn test_odometer_short_read_and_write() {
        let odometer = OdometerShort { data: Some(123456) };
        let bytes = odometer.to_bytes().unwrap();
        assert_eq!(bytes.len(), 3);

        let mut reader = BinMemoryBuffer::from(bytes.clone());
        let parsed = OdometerShort::read(&mut reader).unwrap();
        assert_eq!(parsed, odometer);
    }

    #[test]
    fn test_odometer_short_none() {
        let odometer = OdometerShort { data: None };
        let bytes = odometer.to_bytes().unwrap();
        assert_eq!(bytes, vec![0xFF, 0xFF, 0xFF]);

        let mut reader = BinMemoryBuffer::from(bytes);
        let parsed = OdometerShort::read(&mut reader).unwrap();
        assert_eq!(parsed, odometer);
    }

    #[test]
    fn test_odometer_short_boundary_values() {
        // Max valid operating value: 9_999_999
        let max_valid = OdometerShort { data: Some(9_999_999) };
        assert!(max_valid.validate().is_ok());
        let bytes = max_valid.to_bytes().unwrap();
        let mut reader = BinMemoryBuffer::from(bytes);
        assert_eq!(OdometerShort::read(&mut reader).unwrap(), max_valid);

        // Out-of-range value: 10_000_000
        let invalid = OdometerShort { data: Some(10_000_000) };
        assert!(invalid.validate().is_err());

        // Ensure zero bytes are written to output buffer when validation fails
        let mut buffer = BinMemoryBuffer::new();
        let result = invalid.write(&mut buffer);
        assert!(result.is_err());
        assert_eq!(buffer.as_slice().len(), 0);

        // Value exceeding 24 bits
        let over_24bit = OdometerShort { data: Some(0x1000000) };
        assert!(over_24bit.validate().is_err());
        let mut buffer2 = BinMemoryBuffer::new();
        assert!(over_24bit.write(&mut buffer2).is_err());
        assert_eq!(buffer2.as_slice().len(), 0);
    }
}
