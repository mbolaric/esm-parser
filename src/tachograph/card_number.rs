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

#[derive(Debug, Serialize)]
pub struct CardNumber {
    pub equipment_type: EquipmentType,
    pub number: String,
}

impl CardNumber {
    /// Entity Identification The first 13 characters identify the entity uniquely.
    pub fn get_identification(&self) -> &str {
        if self.number.len() < 13 {
            return "";
        }
        &self.number[0..13]
    }

    /// The 14th character is a sequential number, used when an entity is issued multiple cards.
    /// This helps distinguish between those different cards.
    /// If a company has three driver cards, the first two might have "1" and "2" as the 14th character, while the third would have "3".
    pub fn get_consecutive_index(&self) -> &str {
        if self.number.len() <= 14 {
            return "";
        }
        &self.number[14..15]
    }

    /// Card replacement index: The 15th character in this sequence is a specific indicator of a card replacement.
    /// Each time a driver card is replaced, the value at index 15 is increased. This helps track how many times a card has been replaced.
    pub fn get_replacement_index(&self) -> &str {
        if self.number.len() <= 15 {
            return "";
        }
        &self.number[15..16]
    }
}

impl ReadableWithParams<CardNumber> for CardNumber {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardNumber> {
        let equipment_type = params.equipment_type.clone();
        let number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
        Ok(Self { number, equipment_type })
    }
}
