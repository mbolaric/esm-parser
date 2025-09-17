use std::collections::HashMap;

use log::{debug, trace};
use serde::Serialize;

use crate::gen1::{CardResponseParameterData, Certificate};
use crate::tacho::{
    Card, CardChipIdentification, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser,
    ControlCardActivityRecord, ControlCardApplicationIdentification, ControlCardControlActivityData,
    ControlCardControlActivityDataParams, Identification, IdentificationParams,
};
use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug, Serialize)]
pub struct ControlCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: ControlCardApplicationIdentification,
    pub identification: Option<Identification>,
    pub controller_activity_data: Option<ControlCardControlActivityData<ControlCardActivityRecord>>,
    pub card_certificate: Option<Certificate>,
    pub ca_certificate: Option<Certificate>,
    pub card_notes: String,
}

impl ControlCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: ControlCardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen1,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            identification: None,
            controller_activity_data: None,
            card_certificate: None,
            ca_certificate: None,
            card_notes,
        }
    }
}

impl CardParser<ControlCard> for ControlCard {
    fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<ControlCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            ControlCardApplicationIdentification,
        >(card_data_files)?;

        let mut control_card = ControlCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
        );

        for card_item in card_data_files.iter() {
            debug!("ControlCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    control_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::ControllerActivityData => {
                    let params =
                        ControlCardControlActivityDataParams::new(application_identification.no_of_control_activity_records);
                    control_card.controller_activity_data = Some(ControlCardControlActivityData::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    control_card.card_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::CACertificate => {
                    control_card.ca_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("ControlCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("ControlCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(control_card))
    }
}
