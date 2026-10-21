use crate::tacho::CardChipIdentification;
use crate::writer::{Validatable, Writable, validate_fixed_bytes};

impl Validatable for CardChipIdentification {
    fn validate(&self) -> crate::Result<()> {
        validate_fixed_bytes::<4>("CardChipIdentification.ic_serial_number", &self.ic_serial_number)?;
        validate_fixed_bytes::<4>("CardChipIdentification.ic_manufacturing_references", &self.ic_manufacturing_references)
    }
}

impl Writable for CardChipIdentification {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_all(&self.ic_serial_number)?;
        writer.write_all(&self.ic_manufacturing_references)?;

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
    fn test_card_chip_identification_roundtrip() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let chip = CardChipIdentification::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        chip.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_card_chip_identification_validation_and_zero_byte_emission() {
        let invalid = CardChipIdentification {
            ic_serial_number: vec![1, 2, 3, 4],
            ic_manufacturing_references: vec![5, 6, 7], // 3 bytes instead of 4
            ic_serial_number_hex: String::new(),
            ic_manufacturing_references_hex: String::new(),
        };
        assert!(invalid.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(invalid.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);
    }
}
