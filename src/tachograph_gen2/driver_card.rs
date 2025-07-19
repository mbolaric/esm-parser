use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    Readable, Result,
    gen1::CardApplicationIdentification,
    gen2::CardResponseParameterData,
    tacho::{Card, CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification, TimeReal},
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: CardApplicationIdentification,
    pub card_download: Option<TimeReal>,
    pub card_notes: String,
}

impl DriverCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: CardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self { card_chip_identification, card_icc_identification, application_identification, card_download: None, card_notes }
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &str) -> Result<Box<DriverCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            CardApplicationIdentification,
        >(card_data_files)?;
        debug!("DriverCard::parse - Application Identification: {:?}", application_identification);

        let mut driver_card =
            DriverCard::new(card_chip_identification, card_icc_identification, application_identification, card_notes.to_owned());

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::CardDownload => {
                    driver_card.card_download = Some(TimeReal::read(&mut reader)?);
                }
                // FIXME: we need to parse all cases
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("DriverCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("DriverCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(driver_card))
    }
}
