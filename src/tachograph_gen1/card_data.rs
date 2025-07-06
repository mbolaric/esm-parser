use std::collections::HashMap;

use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::{CardApplicationIdentification, CardResponseParameterData, CompanyCard, ControlCard, DriverCard, WorkshopCard};
use crate::tacho::{
    self, ApplicationIdentification, CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification, EquipmentType,
    TachographHeader,
};
use crate::{Readable, Result};

#[derive(Debug)]
pub struct CardData {
    pub header: TachographHeader,
    pub card_data_responses: CardResponseParameterData,
}

impl CardData {
    pub fn parse_ic(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardChipIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::IC, card_data_files)?;
        let card_chip_identification = CardChipIdentification::read(&mut reader)?;
        Ok(card_chip_identification)
    }

    pub fn parse_icc(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardIccIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::ICC, card_data_files)?;
        let card_icc_identification = CardIccIdentification::read(&mut reader)?;
        Ok(card_icc_identification)
    }

    pub fn parse_card_application_identification<T: Readable<T>>(
        card_data_files: &HashMap<CardFileID, CardDataFile>,
    ) -> Result<T> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::ApplicationIdentification,
            card_data_files,
        )?;
        let application_identification = T::read(&mut reader)?;
        Ok(application_identification)
    }

    fn parse_application_identification(
        card_data_files: &HashMap<CardFileID, CardDataFile>,
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
            &|card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String| {
                CardData::parse_card(card_data_files, card_notes)
            },
        )?;

        debug!("CardData::from_data - Header: {:?}, Note: {:?}", header, card_data_responses);

        Ok(Self { header, card_data_responses })
    }

    fn parse_card(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String) -> Result<CardResponseParameterData> {
        debug!("CardData::parse_card - Data Files Count: {:?}, Note: {:?}", card_data_files.len(), card_notes);
        let application_identification = CardData::parse_application_identification(card_data_files)?;
        debug!("CardData::parse_card - Application identification: {:?}", application_identification);
        // FIXME: Replace Card with concrete card type
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
