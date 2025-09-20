use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::{
    Error, Export, Result, gen1,
    gen2::{self, CardResponseParameterData, ParsedCard},
    tacho::{
        self, CardFilesDataByCardGeneration, CardFilesDataByCardGenerationItem, CardGeneration, CardParser, EquipmentType,
        TachographHeader,
    },
};

#[derive(Debug, Serialize)]
pub struct CardData {
    pub header: TachographHeader,
    #[serde(rename = "cardDataResponses")]
    pub card_data_responses: CardResponseParameterData,
}

impl CardData {
    pub fn from_data<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_data_files: &CardFilesDataByCardGeneration| CardData::parse_card(card_data_files),
        )?;

        debug!("CardData::from_data - Header: {header:?}, Note: {card_data_responses:?}");

        Ok(Self { header, card_data_responses })
    }

    fn get_card_by_equipment_type<TGen1: CardParser<TGen1>, TGen2: CardParser<TGen2>>(
        generation: CardGeneration,
        card_files_data_gen1: &CardFilesDataByCardGenerationItem,
        card_files_data_gen2: &CardFilesDataByCardGenerationItem,
    ) -> Result<ParsedCard<TGen1, TGen2>> {
        if generation == CardGeneration::Combined {
            return Ok(ParsedCard::Combined(
                TGen1::parse(&card_files_data_gen1.card_files_data, &card_files_data_gen1.card_notes)?,
                TGen2::parse(&card_files_data_gen2.card_files_data, &card_files_data_gen2.card_notes)?,
            ));
        }
        if generation == CardGeneration::Gen1 {
            return Ok(ParsedCard::Gen1(TGen1::parse(&card_files_data_gen1.card_files_data, &card_files_data_gen1.card_notes)?));
        }
        Ok(ParsedCard::Gen2(TGen2::parse(&card_files_data_gen2.card_files_data, &card_files_data_gen2.card_notes)?))
    }

    fn parse_card(card_data_files_by_gen: &CardFilesDataByCardGeneration) -> Result<CardResponseParameterData> {
        let generation = card_data_files_by_gen.get_card_generation();
        if generation == CardGeneration::Gen1 {
            return Err(Error::InvalidDataGeneration);
        }

        let card_files_data_gen1 = &card_data_files_by_gen.card_files_data_gen1;
        let card_files_data_gen2 = &card_data_files_by_gen.card_files_data_gen2;

        debug!(
            "CardData::parse_card - Gen1 - Data Files Count: {:?}, Note: {:?}",
            card_files_data_gen1.card_files_data.len(),
            card_files_data_gen1.card_notes
        );
        debug!(
            "CardData::parse_card - Gen2 - Data Files Count: {:?}, Note: {:?}",
            card_files_data_gen2.card_files_data.len(),
            card_files_data_gen2.card_notes
        );
        let application_identification = <dyn tacho::Card<CardResponseParameterData>>::parse_application_identification(
            &card_files_data_gen1.card_files_data,
        )?;
        debug!("CardData::parse_card - Application identification: {application_identification:?}");
        match application_identification.type_of_tachograph_card_id {
            EquipmentType::DriverCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::DriverCard, gen2::DriverCard>(
                    generation,
                    card_files_data_gen1,
                    card_files_data_gen2,
                )?;
                Ok(CardResponseParameterData::DriverCard(cards))
            }
            EquipmentType::CompanyCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::CompanyCard, gen2::CompanyCard>(
                    generation,
                    card_files_data_gen1,
                    card_files_data_gen2,
                )?;
                Ok(CardResponseParameterData::CompanyCard(cards))
            }
            EquipmentType::ControlCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::ControlCard, gen2::ControlCard>(
                    generation,
                    card_files_data_gen1,
                    card_files_data_gen2,
                )?;
                Ok(CardResponseParameterData::ControlCard(cards))
            }
            EquipmentType::WorkshopCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::WorkshopCard, gen2::WorkshopCard>(
                    generation,
                    card_files_data_gen1,
                    card_files_data_gen2,
                )?;
                Ok(CardResponseParameterData::WorkshopCard(cards))
            }
            _ => Ok(CardResponseParameterData::Unsupported),
        }
    }
}

impl Export for CardData {}
