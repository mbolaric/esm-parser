use crate::BCDString;
use crate::tacho::Datef;
use crate::writer::{Validatable, Writable};

impl Validatable for Datef {
    /// Validates the date according to Gregorian calendar rules and EU tachograph conventions:
    /// - Year, month, and day strings must have lengths 4, 2, and 2 respectively.
    /// - All characters must be ASCII decimal digits (for valid BCD encoding).
    /// - "0000-00-00" is recognized as the legal "no date" sentinel.
    /// - Partial zero dates (e.g. "2024-00-15") are rejected as corrupted.
    /// - Month must be in range `1..=12`.
    /// - Day must be in range `1..=max_days` for the month, respecting leap years.
    fn validate(&self) -> crate::Result<()> {
        if self.year.len() != 4 || self.month.len() != 2 || self.day.len() != 2 {
            return Err(crate::Error::ValidationError(format!(
                "Invalid Datef digit lengths: year={}, month={}, day={}",
                self.year.len(),
                self.month.len(),
                self.day.len()
            )));
        }

        if !self.year.chars().all(|c| c.is_ascii_digit())
            || !self.month.chars().all(|c| c.is_ascii_digit())
            || !self.day.chars().all(|c| c.is_ascii_digit())
        {
            return Err(crate::Error::ValidationError(format!("Datef contains non-digit characters: {self}")));
        }

        // Special legal sentinel: "0000-00-00" indicates "no date"
        if self.year == "0000" && self.month == "00" && self.day == "00" {
            return Ok(());
        }

        // Partial zeroes are corrupted/invalid
        if self.year == "0000" || self.month == "00" || self.day == "00" {
            return Err(crate::Error::ValidationError(format!("Invalid partial zero date: {self}")));
        }

        let y: u32 = self.year.parse().map_err(|_| crate::Error::ValidationError("Invalid year".into()))?;
        let m: u32 = self.month.parse().map_err(|_| crate::Error::ValidationError("Invalid month".into()))?;
        let d: u32 = self.day.parse().map_err(|_| crate::Error::ValidationError("Invalid day".into()))?;

        if !(1..=12).contains(&m) {
            return Err(crate::Error::ValidationError(format!("Month {m} out of range 1..=12 in date {self}")));
        }

        // Gregorian leap year rules: divisible by 4, except end-of-century years not divisible by 400
        let is_leap = (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400);
        let max_days = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };

        if !(1..=max_days).contains(&d) {
            return Err(crate::Error::ValidationError(format!(
                "Day {d} out of range 1..={max_days} for month {m} year {y} in date {self}"
            )));
        }

        Ok(())
    }
}

impl Writable for Datef {
    /// Writes a BCD-encoded date to a binary writer (4 bytes total: 2 for year, 1 for month, 1 for day).
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        let year_bytes = BCDString::encode_strict::<2>(&self.year)?;
        let month_bytes = BCDString::encode_strict::<1>(&self.month)?;
        let day_bytes = BCDString::encode_strict::<1>(&self.day)?;

        writer.write_all(&year_bytes)?;
        writer.write_all(&month_bytes)?;
        writer.write_all(&day_bytes)?;
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
    fn test_datef_write() {
        let datef = Datef { year: "2023".to_string(), month: "09".to_string(), day: "11".to_string() };
        let bytes = datef.to_bytes().unwrap();
        assert_eq!(bytes, vec![0x20, 0x23, 0x09, 0x11]);
    }

    #[test]
    fn test_datef_roundtrip() {
        let original_bytes = vec![0x20, 0x24, 0x12, 0x31];
        let mut reader = BinMemoryBuffer::from(original_bytes.clone());
        let parsed = Datef::read(&mut reader).unwrap();
        let written_bytes = parsed.to_bytes().unwrap();
        assert_eq!(written_bytes, original_bytes);
    }

    #[test]
    fn test_datef_calendar_validation() {
        // Valid leap day: 2024-02-29
        let leap_valid = Datef { year: "2024".into(), month: "02".into(), day: "29".into() };
        assert!(leap_valid.validate().is_ok());
        assert_eq!(leap_valid.to_bytes().unwrap(), vec![0x20, 0x24, 0x02, 0x29]);

        // Invalid leap day: 2023-02-29 (not a leap year)
        let leap_invalid = Datef { year: "2023".into(), month: "02".into(), day: "29".into() };
        assert!(leap_invalid.validate().is_err());
        let mut buffer = BinMemoryBuffer::new();
        assert!(leap_invalid.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0); // Atomic zero bytes

        // Invalid day for April: 2024-04-31 (April has 30 days)
        let apr_invalid = Datef { year: "2024".into(), month: "04".into(), day: "31".into() };
        assert!(apr_invalid.validate().is_err());

        // Month 0
        let month_zero = Datef { year: "2024".into(), month: "00".into(), day: "15".into() };
        assert!(month_zero.validate().is_err());

        // Month 13
        let month_13 = Datef { year: "2024".into(), month: "13".into(), day: "01".into() };
        assert!(month_13.validate().is_err());

        // Day 0
        let day_zero = Datef { year: "2024".into(), month: "05".into(), day: "00".into() };
        assert!(day_zero.validate().is_err());

        // Legal sentinel "0000-00-00" (no date)
        let sentinel = Datef { year: "0000".into(), month: "00".into(), day: "00".into() };
        assert!(sentinel.validate().is_ok());
        assert_eq!(sentinel.to_bytes().unwrap(), vec![0x00, 0x00, 0x00, 0x00]);

        // Partial zero date: "0000-05-12"
        let partial_zero = Datef { year: "0000".into(), month: "05".into(), day: "12".into() };
        assert!(partial_zero.validate().is_err());
    }
}
