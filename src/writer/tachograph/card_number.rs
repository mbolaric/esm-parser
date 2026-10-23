use binary_data::{BinSeek, WriteBytes};

use crate::tacho::{CardNumber, EquipmentType};
use crate::writer::{Validatable, Writable, encode_ia5_padded, validate_ia5_string};
use crate::{Error, Result};

impl CardNumber {
    fn encoded_number(&self) -> Result<[u8; CardNumber::CARD_NUMBER_LENGTH as usize]> {
        if !self.number.is_empty() {
            return encode_ia5_padded("CardNumber", &self.number);
        }
        let mut number = [b' '; CardNumber::CARD_NUMBER_LENGTH as usize];
        if self.identification.is_empty()
            && self.card_consecutive_index.is_empty()
            && self.card_replacement_index.is_empty()
            && self.card_renewal_index.is_empty()
        {
            return Ok(number);
        }
        match self.equipment_type {
            EquipmentType::DriverCard => {
                validate_ia5_string::<0>("CardNumber.card_consecutive_index", &self.card_consecutive_index)?;
                number[..14].copy_from_slice(&encode_ia5_padded::<14>("CardNumber.identification", &self.identification)?);
            }
            EquipmentType::WorkshopCard | EquipmentType::ControlCard | EquipmentType::CompanyCard => {
                number[..13].copy_from_slice(&encode_ia5_padded::<13>("CardNumber.identification", &self.identification)?);
                number[13] = encode_ia5_padded::<1>("CardNumber.card_consecutive_index", &self.card_consecutive_index)?[0];
            }
            _ => {
                return Err(Error::ValidationError(format!(
                    "Cannot construct CardNumber components for {:?}",
                    self.equipment_type
                )));
            }
        }
        number[14] = encode_ia5_padded::<1>("CardNumber.card_replacement_index", &self.card_replacement_index)?[0];
        number[15] = encode_ia5_padded::<1>("CardNumber.card_renewal_index", &self.card_renewal_index)?[0];
        Ok(number)
    }
}

impl Validatable for CardNumber {
    fn validate(&self) -> Result<()> {
        self.encoded_number().map(|_| ())
    }
}

impl Writable for CardNumber {
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.encoded_number()?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::{BinMemoryBuffer, BinSeek};

    use crate::ReadableWithParams;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    #[test]
    fn test_card_number_roundtrip() {
        let original_bytes = b"ABC1234567890123";
        let mut reader = BinMemoryBuffer::from(original_bytes.to_vec());
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let card_number = CardNumber::read(&mut reader, &params).unwrap();

        let mut writer = BinMemoryBuffer::new();
        card_number.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), original_bytes);

        // Also test short padding
        let short_card = CardNumber {
            equipment_type: EquipmentType::DriverCard,
            number: "ABC1234".to_string(),
            identification: "ABC1234".to_string(),
            card_replacement_index: "".to_string(),
            card_consecutive_index: "".to_string(),
            card_renewal_index: "".to_string(),
        };
        let mut writer_short = BinMemoryBuffer::new();
        short_card.write(&mut writer_short).unwrap();
        assert_eq!(writer_short.len().unwrap(), 16);
        assert_eq!(&writer_short.as_slice()[0..7], b"ABC1234");
        assert_eq!(&writer_short.as_slice()[7..16], b"         ");
    }

    #[test]
    fn test_card_number_validation_and_zero_byte_emission() {
        let overlong = CardNumber {
            equipment_type: EquipmentType::DriverCard,
            number: "A".repeat(17),
            identification: String::new(),
            card_replacement_index: String::new(),
            card_consecutive_index: String::new(),
            card_renewal_index: String::new(),
        };
        assert!(overlong.validate().is_err());
        let mut writer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        let non_ascii = CardNumber {
            equipment_type: EquipmentType::DriverCard,
            number: "ABC123456789012Ä".to_string(),
            identification: String::new(),
            card_replacement_index: String::new(),
            card_consecutive_index: String::new(),
            card_renewal_index: String::new(),
        };
        assert!(non_ascii.validate().is_err());
        let mut writer2 = BinMemoryBuffer::new();
        assert!(non_ascii.write(&mut writer2).is_err());
        assert_eq!(writer2.as_slice().len(), 0);
    }
}
