use std::collections::HashMap;

use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen1::{CardResponseParameterData, CompanyCard, ControlCard, DriverCard, WorkshopCard};
use crate::tacho::{self, CardDataFile, CardFileID, EquipmentType, TachographHeader};

#[derive(Debug)]
pub struct CardData {
    pub header: TachographHeader,
    pub card_data_responses: CardResponseParameterData,
}

impl CardData {
    pub fn from_data<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String| {
                CardData::parse_card(card_data_files, card_notes)
            },
        )?;

        debug!("CardData::from_data - Header: {:?}, Note: {:?}", header, card_data_responses);

        Ok(Self { header, card_data_responses })
    }

    fn parse_card(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String) -> Result<CardResponseParameterData> {
        debug!("CardData::parse_card - Data Files Count: {:?}, Note: {:?}", card_data_files.len(), card_notes);
        let application_identification =
            <dyn tacho::Card<CardResponseParameterData>>::parse_application_identification(card_data_files)?;
        debug!("CardData::parse_card - Application identification: {:?}", application_identification);
        match application_identification.type_of_tachograph_card_id {
            EquipmentType::DriverCard => {
                Ok(CardResponseParameterData::DriverCard(DriverCard::parse(card_data_files, card_notes)?))
            }
            EquipmentType::CompanyCard => {
                Ok(CardResponseParameterData::CompanyCard(CompanyCard::parse(card_data_files, card_notes)?))
            }
            EquipmentType::ControlCard => {
                Ok(CardResponseParameterData::ControlCard(ControlCard::parse(card_data_files, card_notes)?))
            }
            EquipmentType::WorkshopCard => {
                Ok(CardResponseParameterData::WorkshopCard(WorkshopCard::parse(card_data_files, card_notes)?))
            }
            _ => Ok(CardResponseParameterData::Unsupported),
        }
    }
}
