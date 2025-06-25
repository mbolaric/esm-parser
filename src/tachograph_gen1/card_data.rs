use std::collections::HashMap;

use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::CardResponseParameterData;
use crate::tacho::{self, CardDataFile, CardFileID, CardItem, TachographHeader};
use crate::{Error, Result};

#[derive(Debug)]
pub struct CardData {
    header: TachographHeader,
    card_data_responses: CardResponseParameterData,
}

impl CardData {
    fn parse_ic(card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>) -> Result<()> {
        let reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::IC,
            &card_data_files,
        )?;

        Ok(())
    }

    fn parse_icc(card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>) -> Result<()> {
        let reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::ICC,
            &card_data_files,
        )?;

        Ok(())
    }

    fn parse_application_identification(
        card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>,
    ) -> Result<()> {
        let reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::ApplicationIdentification,
            &card_data_files,
        )?;

        Ok(())
    }

    pub fn from_data<R: ReadBytes + BinSeek>(
        header: TachographHeader,
        reader: &mut R,
    ) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>,
              card_notes: &String| CardData::parse_card(card_data_files, card_notes),
        )?;

        debug!(
            "CardData::from_data - {:?}, Note: {:?}",
            header, card_data_responses
        );

        Ok(Self {
            header,
            card_data_responses,
        })
    }

    fn parse_card(
        card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>,
        card_notes: &String,
    ) -> Result<CardResponseParameterData> {
        debug!(
            "CardData::parse_card - {:?}, Note: {:?}",
            card_data_files, card_notes
        );

        // FIXME:
        let _ = CardData::parse_ic(&card_data_files)?;
        let _ = CardData::parse_icc(&card_data_files)?;
        let _ = CardData::parse_application_identification(&card_data_files)?;

        Ok(CardResponseParameterData::DriverCard)
    }
}
