use crate::tacho::{SpecificConditionRecord, SpecificConditionType};
use crate::writer::{Validatable, Writable};

impl Validatable for SpecificConditionRecord {
    fn validate(&self) -> crate::Result<()> {
        self.entry_time.validate()?;
        if self.specific_condition_type == SpecificConditionType::Unknown {
            return Err(crate::Error::ValidationError("SpecificConditionRecord has unknown specific_condition_type".into()));
        }
        Ok(())
    }
}

impl Writable for SpecificConditionRecord {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.entry_time.write_raw(writer)?;
        writer.write_u8(self.specific_condition_type as u8)?;
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
    fn test_specific_condition_record_roundtrip() {
        let mut data = vec![0x00, 0x00, 0x01, 0x00]; // entryTime
        data.push(0x01); // specificConditionType

        let mut reader = BinMemoryBuffer::from(data.clone());
        let record = SpecificConditionRecord::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        record.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_specific_condition_record_validation_and_zero_byte_emission() {
        let invalid =
            SpecificConditionRecord { entry_time: TimeReal::new(256), specific_condition_type: SpecificConditionType::Unknown };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
