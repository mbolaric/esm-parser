use binary_data::{BinSeek, ReadBytes};

use crate::{Error, Readable, ReadableWithParams, Result, helpers::vec_u8_to_string, tacho::EquipmentType};

#[derive(Debug)]
pub struct CardNumberParams {
    pub equipment_type: EquipmentType,
}

impl CardNumberParams {
    pub fn new(equipment_type: EquipmentType) -> Self {
        Self { equipment_type }
    }
}

pub enum CardNumber {
    Driver(CardNumberDriver),
    Owner(CardNumberOwner),
}

impl ReadableWithParams<CardNumber> for CardNumber {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardNumber> {
        let equipment_type = &params.equipment_type;
        let number = vec_u8_to_string(reader.read_into_vec(16)?)?;
        match equipment_type {
            EquipmentType::DriverCard => Ok(CardNumber::Driver(CardNumberDriver { number })),
            EquipmentType::ControlCard => Ok(CardNumber::Owner(CardNumberOwner { number })),
            _ => Err(Error::NotImplemented),
        }
    }
}

#[derive(Debug)]
pub struct CardNumberOwner {
    pub number: String,
}

impl CardNumberOwner {
    pub fn owner_identification(&self) -> &str {
        if self.number.len() < 13 {
            return "";
        }
        &self.number[0..13]
    }
}

impl Readable<CardNumberOwner> for CardNumberOwner {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardNumberOwner> {
        let number = vec_u8_to_string(reader.read_into_vec(16)?)?;
        Ok(Self { number })
    }
}

#[derive(Debug)]
pub struct CardNumberDriver {
    pub number: String,
}

impl CardNumberDriver {
    pub fn get_card_replacement_index(&self) -> &str {
        if self.number.len() <= 14 {
            return "";
        }
        &self.number[14..15]
    }

    pub fn get_card_renewal_index(&self) -> &str {
        if self.number.len() <= 15 {
            return "";
        }
        &self.number[15..16]
    }
}

impl Readable<CardNumberDriver> for CardNumberDriver {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardNumberDriver> {
        let number = vec_u8_to_string(reader.read_into_vec(16)?)?;
        Ok(Self { number })
    }
}
