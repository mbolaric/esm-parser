use std::collections::HashMap;

use log::debug;

use crate::Result;
use crate::gen1::{CardApplicationIdentification, CardData};
use crate::tacho::{CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification};

#[derive(Debug)]
pub struct WorkshopCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: CardApplicationIdentification,
    pub card_notes: String,
}

impl WorkshopCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: CardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self {
            card_chip_identification,
            card_icc_identification,
            application_identification,
            // card_download: None,
            // card_event_data: None,
            // card_driving_license_info: None,
            // card_fault_data: None,
            // identification: None,
            // card_driver_activity: None,
            // specific_conditions: None,
            // control_activity_data: None,
            // current_usage: None,
            // vehicles_used: None,
            // card_places: None,
            card_notes,
        }
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String) -> Result<Box<WorkshopCard>> {
        let card_chip_identification = CardData::parse_ic(card_data_files)?;
        let card_icc_identification = CardData::parse_icc(card_data_files)?;
        let application_identification = CardData::parse_card_application_identification(card_data_files)?;
        debug!("WorkshopCard::parse - Application Identification: {:?}", application_identification);

        let driver_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.clone(),
        );

        // FIXME:

        Ok(Box::new(driver_card))
    }
}
