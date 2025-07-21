use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    Error, Result, gen1,
    gen2::{self, CardResponseParameterData},
    tacho::{self, CardDataFilesByCardGeneration, CardGeneration, EquipmentType, TachographHeader},
};

#[derive(Debug)]
pub struct CardData {
    pub header: TachographHeader,
    pub card_data_responses: CardResponseParameterData,
}

impl CardData {
    pub fn from_data<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_data_files: &CardDataFilesByCardGeneration| CardData::parse_card(card_data_files),
        )?;

        debug!("CardData::from_data - Header: {:?}, Note: {:?}", header, card_data_responses);

        Ok(Self { header, card_data_responses })
    }

    fn parse_card(card_data_files_by_gen: &CardDataFilesByCardGeneration) -> Result<CardResponseParameterData> {
        let generation = card_data_files_by_gen.get_card_generation();
        if generation == CardGeneration::Gen1 {
            return Err(Error::InvalidDataGeneration);
        }

        let card_data_files_gen1 = &card_data_files_by_gen.card_data_files_gen1.card_data_files;
        let card_notes_gen1 = &card_data_files_by_gen.card_data_files_gen1.card_notes;
        let card_data_files_gen2 = &card_data_files_by_gen.card_data_files_gen2.card_data_files;
        let card_notes_gen2 = &card_data_files_by_gen.card_data_files_gen2.card_notes;

        debug!("CardData::parse_card - Gen1 - Data Files Count: {:?}, Note: {:?}", card_data_files_gen1.len(), card_notes_gen1);
        debug!("CardData::parse_card - Gen2 - Data Files Count: {:?}, Note: {:?}", card_data_files_gen2.len(), card_notes_gen2);
        let application_identification =
            <dyn tacho::Card<CardResponseParameterData>>::parse_application_identification(card_data_files_gen1)?;
        debug!("CardData::parse_card - Application identification: {:?}", application_identification);
        // FIXME: Replace Card with concrete card type
        match application_identification.type_of_tachograph_card_id {
            EquipmentType::DriverCard => {
                if generation == CardGeneration::Combined {
                    return Ok(CardResponseParameterData::DriverCard(
                        Some(gen1::DriverCard::parse(card_data_files_gen1, card_notes_gen1)?),
                        Some(gen2::DriverCard::parse(card_data_files_gen2, card_notes_gen2)?),
                    ));
                }
                if generation == CardGeneration::Gen1 {
                    return Ok(CardResponseParameterData::DriverCard(
                        Some(gen1::DriverCard::parse(card_data_files_gen1, card_notes_gen1)?),
                        None,
                    ));
                }
                Ok(CardResponseParameterData::DriverCard(
                    None,
                    Some(gen2::DriverCard::parse(card_data_files_gen2, card_notes_gen2)?),
                ))
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
                Err(Error::NotImplemented)
                // Ok(CardResponseParameterData::WorkshopCard(WorkshopCard::parse(card_data_files, card_notes)?))
            }
            _ => Ok(CardResponseParameterData::Unsupported),
        }
    }
}
