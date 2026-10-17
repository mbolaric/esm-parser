use binary_data::{BinSeek, ReadBytes, WriteBytes};
use serde::Serialize;

use crate::tacho::EquipmentType;
use crate::{ReadableWithParams, Result, Validatable, Writable, bytes_to_ia5_fix_string, validate_ia5_string, write_ia5_string};

const CARD_NUMBER_LENGTH: u32 = 16;

#[derive(Debug)]
pub struct CardNumberParams {
    pub equipment_type: EquipmentType,
}

impl CardNumberParams {
    pub fn new(equipment_type: EquipmentType) -> Self {
        Self { equipment_type }
    }
}

/// A card number as defined by definition g).
/// CardNumber ::~ CHOICE {
///     SEQUENCE {
///         driveridentification    IA5String(SIZE(14)),
///         cardReplacementindex    CardReplacementindex,
///         cardRenewalindex        CardRenewalindex
///     },
///     SEQUENCE {
///         owneridentification     IA5String(SIZE(13)),
///         cardConsecutiveindex    CardConsecutiveindex,
///         cardReplacementindex    CardReplacementindex,
///         cardRenewalindex        CardRenewalindex
///     }
/// }
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardNumber {
    #[serde(rename = "cardIssuingMemberState")]
    pub equipment_type: EquipmentType,
    pub number: String,
    pub identification: String,
    #[serde(rename = "cardReplacementindex")]
    pub card_replacement_index: String,
    #[serde(rename = "cardConsecutiveindex")]
    pub card_consecutive_index: String,
    #[serde(rename = "cardRenewalindex")]
    pub card_renewal_index: String,
}

impl CardNumber {
    fn effective_number(&self) -> String {
        if !self.number.is_empty() {
            self.number.clone()
        } else if !self.identification.is_empty() {
            format!(
                "{}{}{}{}",
                self.identification, self.card_consecutive_index, self.card_replacement_index, self.card_renewal_index
            )
        } else {
            String::new()
        }
    }

    /// Entity Identification The first 13 or 14 characters identify the entity uniquely.
    fn get_identification<'a>(number: &'a str, equipment_type: &EquipmentType) -> &'a str {
        if number.len() < 13 {
            return "";
        }
        match equipment_type {
            EquipmentType::DriverCard => &number[0..14],
            _ => &number[0..13],
        }
    }

    /// The 14th or 15th character is a sequential number, used when an entity is issued multiple cards.
    /// This helps distinguish between those different cards.
    /// If a company has three driver cards, the first two might have "1" and "2" as the 14th character, while the third would have "3".
    fn get_consecutive_index<'a>(number: &'a str, equipment_type: &EquipmentType) -> &'a str {
        if number.len() <= 14 {
            return "";
        }
        match equipment_type {
            EquipmentType::DriverCard => "",
            _ => &number[13..14],
        }
    }

    /// Card replacement index: The 15th or 16th character in this sequence is a specific indicator of a card replacement.
    /// Each time a driver card is replaced, the value at index 15 is increased. This helps track how many times a card has been replaced.
    fn get_replacement_index(number: &str) -> &str {
        if number.len() <= 14 {
            return "";
        }
        &number[14..15]
    }

    fn get_renewal_index(number: &str) -> &str {
        if number.len() <= 15 {
            return "";
        }
        &number[15..16]
    }
}

impl Validatable for CardNumber {
    fn validate(&self) -> Result<()> {
        validate_ia5_string::<{ CARD_NUMBER_LENGTH as usize }>("CardNumber", &self.effective_number())
    }
}

impl ReadableWithParams for CardNumber {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardNumber> {
        let equipment_type = params.equipment_type;
        let number = bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_NUMBER_LENGTH)?)?;
        let identification = CardNumber::get_identification(&number, &equipment_type).to_string();
        let card_replacement_index = CardNumber::get_replacement_index(&number).to_string();
        let card_consecutive_index = CardNumber::get_consecutive_index(&number, &equipment_type).to_string();
        let card_renewal_index = CardNumber::get_renewal_index(&number).to_string();
        Ok(Self { number, equipment_type, identification, card_replacement_index, card_consecutive_index, card_renewal_index })
    }
}

impl Writable for CardNumber {
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        write_ia5_string::<{ CARD_NUMBER_LENGTH as usize }, _>(writer, "CardNumber", &self.effective_number())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_read_driver_card_number() {
        let data: &[u8] = b"ABC1234567890123";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let card_number = CardNumber::read(&mut reader, &params).unwrap();

        assert_eq!(card_number.number, "ABC1234567890123");
        assert_eq!(card_number.identification, "ABC12345678901");
        assert_eq!(card_number.card_consecutive_index, "");
        assert_eq!(card_number.card_replacement_index, "2");
        assert_eq!(card_number.card_renewal_index, "3");
    }

    #[test]
    fn test_read_workshop_card_number() {
        let data: &[u8] = b"WSP9876543210987";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::WorkshopCard);
        let card_number = CardNumber::read(&mut reader, &params).unwrap();

        assert_eq!(card_number.number, "WSP9876543210987");
        assert_eq!(card_number.identification, "WSP9876543210");
        assert_eq!(card_number.card_consecutive_index, "9");
        assert_eq!(card_number.card_replacement_index, "8");
        assert_eq!(card_number.card_renewal_index, "7");
    }

    #[test]
    fn test_read_short_card_number() {
        let data: &[u8] = b"SHORT";
        let mut reader = BinMemoryBuffer::from(data.to_vec());
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let result = CardNumber::read(&mut reader, &params);
        assert!(result.is_err());
    }

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
