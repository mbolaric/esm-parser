use crate::tacho::FullCardNumber;
use crate::writer::{Validatable, Writable, validate_ia5_string, write_ia5_string};

impl Validatable for FullCardNumber {
    fn validate(&self) -> crate::Result<()> {
        validate_ia5_string::<{ FullCardNumber::CARD_NUMBER_LENGTH as usize }>("FullCardNumber.card_number", &self.card_number)
    }
}

impl Writable for FullCardNumber {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.card_type as u8)?;
        writer.write_u8(self.card_issuing_member_state as u8)?;
        write_ia5_string::<{ FullCardNumber::CARD_NUMBER_LENGTH as usize }, _>(
            writer,
            "FullCardNumber.card_number",
            &self.card_number,
        )
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    #[test]
    fn test_full_card_number_roundtrip() {
        let mut data = vec![0x01, 0x01]; // DriverCard, Austria
        let card_str = "A123456789012345";
        data.extend_from_slice(card_str.as_bytes());

        let mut reader = BinMemoryBuffer::from(data.clone());
        let full = FullCardNumber::read(&mut reader).unwrap();

        let mut writer = BinMemoryBuffer::new();
        full.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_full_card_number_validation_and_zero_byte_emission() {
        let overlong = FullCardNumber {
            card_type: EquipmentType::DriverCard,
            card_issuing_member_state: NationNumeric::Austria,
            card_number: "A".repeat(17),
        };
        assert!(overlong.validate().is_err());

        let mut writer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        let unencodable = FullCardNumber {
            card_type: EquipmentType::DriverCard,
            card_issuing_member_state: NationNumeric::Austria,
            card_number: "CARD-€".into(),
        };
        assert!(unencodable.validate().is_err());

        let mut writer2 = BinMemoryBuffer::new();
        assert!(unencodable.write(&mut writer2).is_err());
        assert_eq!(writer2.as_slice().len(), 0);
    }
}
