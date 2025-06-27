use std::collections::HashMap;

use log::trace;

use crate::{
    Readable, Result,
    gen1::CardResponseParameterData,
    tacho::{self, CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification},
};

#[derive(Debug)]
pub struct Card {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
}

impl Card {
    fn parse_ic(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardChipIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::IC, card_data_files)?;
        let card_chip_identification = CardChipIdentification::read(&mut reader)?;
        Ok(card_chip_identification)
    }

    fn parse_icc(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardIccIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::ICC, card_data_files)?;
        let card_icc_identification = CardIccIdentification::read(&mut reader)?;
        Ok(card_icc_identification)
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<Card> {
        let card_chip_identification = Card::parse_ic(card_data_files)?;
        let card_icc_identification = Card::parse_icc(card_data_files)?;

        for card_item in card_data_files.iter() {
            match card_item.0 {
                _ => trace!("Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Self { card_chip_identification, card_icc_identification })
    }
}
