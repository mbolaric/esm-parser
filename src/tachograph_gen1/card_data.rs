use std::collections::HashMap;

use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::{Card, CardResponseParameterData};
use crate::tacho::{self, ApplicationIdentification, CardDataFile, CardFileID, CardItem, EquipmentType, TachographHeader};
use crate::{Readable, Result};

#[derive(Debug)]
pub struct CardData {
    pub header: TachographHeader,
    pub card_data_responses: CardResponseParameterData,
}

impl CardData {
    fn parse_application_identification(
        card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>,
    ) -> Result<ApplicationIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::ApplicationIdentification,
            card_data_files,
        )?;
        let application_identification = ApplicationIdentification::read(&mut reader)?;
        Ok(application_identification)
    }

    pub fn from_data<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>, card_notes: &String| {
                CardData::parse_card(card_data_files, card_notes)
            },
        )?;

        debug!("CardData::from_data - {:?}, Note: {:?}", header, card_data_responses);

        Ok(Self { header, card_data_responses })
    }

    fn parse_card(
        card_data_files: &HashMap<CardFileID, CardItem<CardDataFile>>,
        card_notes: &String,
    ) -> Result<CardResponseParameterData> {
        debug!("CardData::parse_card - {:?}, Note: {:?}", card_data_files, card_notes);
        let application_identification = CardData::parse_application_identification(card_data_files)?;
        debug!("CardData::parse_card - Application identification: {:?}", application_identification);
        // FIXME: Replace Card with concrete card type
        match application_identification.type_of_tachograph_card_id {
            EquipmentType::DriverCard => Ok(CardResponseParameterData::DriverCard(Card::parse(card_data_files)?)),
            EquipmentType::CompanyCard => Ok(CardResponseParameterData::CompanyCard(Card::parse(card_data_files)?)),
            EquipmentType::ControlCard => Ok(CardResponseParameterData::ControlCard(Card::parse(card_data_files)?)),
            EquipmentType::WorkshopCard => Ok(CardResponseParameterData::WorkshopCard(Card::parse(card_data_files)?)),
            _ => Ok(CardResponseParameterData::Unsupported),
        }
    }
}
