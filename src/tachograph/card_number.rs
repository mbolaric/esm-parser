use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{ReadableWithParams, Result, bytes_to_ia5_fix_string, tacho::EquipmentType};

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
#[derive(Debug, Serialize)]
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

impl ReadableWithParams<CardNumber> for CardNumber {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardNumber> {
        let equipment_type = params.equipment_type.clone();
        let number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
        let identification = CardNumber::get_identification(&number, &equipment_type).to_string();
        let card_replacement_index = CardNumber::get_replacement_index(&number).to_string();
        let card_consecutive_index = CardNumber::get_consecutive_index(&number, &equipment_type).to_string();
        let card_renewal_index = CardNumber::get_renewal_index(&number).to_string();
        Ok(Self { number, equipment_type, identification, card_replacement_index, card_consecutive_index, card_renewal_index })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_data::BinMemoryBuffer;

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
}
