use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    Error, Result, gen1,
    gen2::{self, CardResponseParameterData},
    tacho::{self, CardFilesDataByCardGeneration, CardFilesMap, CardGeneration, CardParser, EquipmentType, TachographHeader},
};

type CardByEquipmentTypeResult<TGen1, TGen2> = (Option<Box<TGen1>>, Option<Box<TGen2>>);

#[derive(Debug)]
pub struct CardData {
    pub header: TachographHeader,
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
        card_files_data_gen1: &CardFilesMap,
        card_notes_gen1: &str,
        card_files_data_gen2: &CardFilesMap,
        card_notes_gen2: &str,
    ) -> Result<CardByEquipmentTypeResult<TGen1, TGen2>> {
        if generation == CardGeneration::Combined {
            return Ok((
                Some(TGen1::parse(card_files_data_gen1, card_notes_gen1)?),
                Some(TGen2::parse(card_files_data_gen2, card_notes_gen2)?),
            ));
        }
        if generation == CardGeneration::Gen1 {
            return Ok((Some(TGen1::parse(card_files_data_gen1, card_notes_gen1)?), None));
        }
        Ok((None, Some(TGen2::parse(card_files_data_gen2, card_notes_gen2)?)))
    }

    fn parse_card(card_data_files_by_gen: &CardFilesDataByCardGeneration) -> Result<CardResponseParameterData> {
        let generation = card_data_files_by_gen.get_card_generation();
        if generation == CardGeneration::Gen1 {
            return Err(Error::InvalidDataGeneration);
        }

        let card_files_data_gen1 = &card_data_files_by_gen.card_files_data_gen1.card_files_data;
        let card_notes_gen1 = &card_data_files_by_gen.card_files_data_gen1.card_notes;
        let card_files_data_gen2 = &card_data_files_by_gen.card_files_data_gen2.card_files_data;
        let card_notes_gen2 = &card_data_files_by_gen.card_files_data_gen2.card_notes;

        debug!("CardData::parse_card - Gen1 - Data Files Count: {:?}, Note: {:?}", card_files_data_gen1.len(), card_notes_gen1);
        debug!("CardData::parse_card - Gen2 - Data Files Count: {:?}, Note: {:?}", card_files_data_gen2.len(), card_notes_gen2);
        let application_identification =
            <dyn tacho::Card<CardResponseParameterData>>::parse_application_identification(card_files_data_gen1)?;
        debug!("CardData::parse_card - Application identification: {application_identification:?}");
        // FIXME: Replace Card with concrete card type
        match application_identification.type_of_tachograph_card_id {
            EquipmentType::DriverCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::DriverCard, gen2::DriverCard>(
                    generation,
                    card_files_data_gen1,
                    card_notes_gen1,
                    card_files_data_gen2,
                    card_notes_gen2,
                )?;
                Ok(CardResponseParameterData::DriverCard(cards.0, cards.1))
            }
            EquipmentType::CompanyCard => {
                Err(Error::NotImplemented)
                // Ok(CardResponseParameterData::CompanyCard(CompanyCard::parse(card_data_files, card_notes)?))
            }
            EquipmentType::ControlCard => {
                Err(Error::NotImplemented)
                // Ok(CardResponseParameterData::ControlCard(ControlCard::parse(card_data_files, card_notes)?))
            }
            EquipmentType::WorkshopCard => {
                let cards = CardData::get_card_by_equipment_type::<gen1::WorkshopCard, gen2::WorkshopCard>(
                    generation,
                    card_files_data_gen1,
                    card_notes_gen1,
                    card_files_data_gen2,
                    card_notes_gen2,
                )?;
                Ok(CardResponseParameterData::WorkshopCard(cards.0, cards.1))
            }
            _ => Ok(CardResponseParameterData::Unsupported),
        }
    }
}
