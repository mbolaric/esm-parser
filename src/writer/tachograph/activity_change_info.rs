use binary_data::{BigEndian, BinSeek, WriteBytes};

use crate::Result;
use crate::tacho::ActivityChangeInfo;
use crate::writer::{Validatable, Writable};

impl Validatable for ActivityChangeInfo {
    /// Validates that the time in minutes since midnight is in `0..1440` (`0..=1439`)
    /// and matches the bits encoded in `activity_info`.
    fn validate(&self) -> Result<()> {
        let encoded_minutes = self.activity_info & ActivityChangeInfo::TIME_MASK;
        if self.time_in_min >= 1440 || encoded_minutes >= 1440 {
            return Err(crate::Error::ValidationError(format!(
                "ActivityChangeInfo time_in_min ({}) out of range 0..1440",
                self.time_in_min.max(encoded_minutes)
            )));
        }
        if self.time_in_min != encoded_minutes {
            return Err(crate::Error::ValidationError(format!(
                "ActivityChangeInfo time_in_min ({}) does not match activity_info bits ({})",
                self.time_in_min, encoded_minutes
            )));
        }
        Ok(())
    }
}

impl Writable for ActivityChangeInfo {
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        writer.write_u16::<BigEndian>(self.activity_info)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ReadableWithParams;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    const REST: u16 = 0b00;
    const DRIVING: u16 = 0b11;

    // Builds the word 'scpaattttttttttt'B from its named bits.
    fn word(slot: u16, status: u16, card_status: u16, activity: u16, minutes: u16) -> u16 {
        (slot << 15) | (status << 14) | (card_status << 13) | (activity << 11) | minutes
    }

    fn decoded(activity_card: ActivityCard, activity_info: u16) -> ActivityChangeInfo {
        ActivityChangeInfo::new(activity_card, activity_info)
    }

    #[test]
    fn decodes_the_last_minute_of_the_day() {
        let info = decoded(ActivityCard::Card, word(0, 0, 0, REST, 1439));
        assert_eq!(info.time_in_min, 1439);
        assert!(info.validate().is_ok());
    }

    #[test]
    fn preserves_an_out_of_range_minute_for_caller_validation() {
        use binary_data::BinMemoryBuffer;

        let info = ActivityChangeInfo::new(ActivityCard::Card, word(0, 0, 0, REST, 1440));
        assert_eq!(info.time_in_min, 1440);
        assert!(info.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(info.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }

    #[test]
    fn test_activity_change_info_roundtrip() {
        use binary_data::BinMemoryBuffer;

        let original_word = word(1, 1, 0, DRIVING, 480);
        let info = ActivityChangeInfo::new(ActivityCard::Card, original_word);
        let mut writer = BinMemoryBuffer::new();
        info.write(&mut writer).unwrap();

        assert_eq!(writer.as_slice(), &original_word.to_be_bytes());

        let mut reader = BinMemoryBuffer::from(writer.as_slice().to_vec());
        let params = ActivityChangeInfoParams::new(ActivityCard::Card);
        let read_back = ActivityChangeInfo::read(&mut reader, &params).unwrap();
        assert_eq!(read_back.activity_info, original_word);
        assert_eq!(read_back.time_in_min, 480);
    }
}
