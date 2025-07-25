use std::collections::HashMap;

use crate::{
    Result,
    gen2::{CardResponseParameterData, WorkshopCardApplicationIdentification},
    tacho::{Card, CardChipIdentification, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser},
};

#[derive(Debug)]
pub struct WorkshopCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: WorkshopCardApplicationIdentification,
    pub card_notes: String,
}

impl WorkshopCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: WorkshopCardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen2,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            card_notes,
        }
    }
}

impl CardParser<WorkshopCard> for WorkshopCard {
    fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<WorkshopCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            WorkshopCardApplicationIdentification,
        >(card_data_files)?;

        let workshop_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
        );

        // FIXME:
        Ok(Box::new(workshop_card))
    }
}
