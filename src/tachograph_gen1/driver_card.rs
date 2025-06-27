use std::collections::HashMap;

use log::trace;

use crate::{
    Error, Readable, Result,
    gen1::{CardResponseParameterData, DriverCardApplicationIdentification},
    tacho::{self, CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification},
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: DriverCardApplicationIdentification,
}

impl DriverCard {
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

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<Box<DriverCard>> {
        let card_chip_identification = DriverCard::parse_ic(card_data_files)?;
        let card_icc_identification = DriverCard::parse_icc(card_data_files)?;

        let mut application_identification: Option<DriverCardApplicationIdentification> = None;

        for card_item in card_data_files.iter() {
            let card_file = card_item.1;
            match card_item.0 {
                CardFileID::ApplicationIdentification => {
                    application_identification =
                        Some(DriverCardApplicationIdentification::read(&mut card_file.data_into_reader()?)?);
                }
                CardFileID::IC | CardFileID::ICC => trace!("Already parsed: {:?}", card_item.0),
                _ => trace!("Not Parsed: {:?}", card_item.0),
            }
        }

        // ApplicationIdentification is always there
        if application_identification.is_none() {
            return Err(Error::MissingCardFile(CardFileID::ApplicationIdentification.to_string()));
        }

        Ok(Box::new(Self {
            card_chip_identification,
            card_icc_identification,
            application_identification: application_identification.unwrap(),
        }))
    }
}
