use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    ReadableWithParams, Result,
    gen2::{
        CardResponseParameterData, Certificate, CertificateParams, WorkshopCardApplicationIdentification,
        WorkshopCardCalibrationRecord,
    },
    tacho::{
        Card, CardChipIdentification, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser,
        Identification, IdentificationParams, WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams,
    },
};

#[derive(Debug)]
pub struct WorkshopCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: WorkshopCardApplicationIdentification,
    pub card_calibration_data: Option<WorkshopCardCalibrationData<WorkshopCardCalibrationRecord>>,
    pub identification: Option<Identification>,
    pub card_certificate: Option<Certificate>,
    pub ca_certificate: Option<Certificate>,
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
            card_calibration_data: None,
            identification: None,
            card_certificate: None,
            ca_certificate: None,
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

        let mut workshop_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
        );

        for card_item in card_data_files.iter() {
            debug!("WorkshopCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::Calibration => {
                    let params = WorkshopCardCalibrationDataParams::new(application_identification.no_off_calibration_records);
                    workshop_card.card_calibration_data =
                        Some(WorkshopCardCalibrationData::<WorkshopCardCalibrationRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    workshop_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.card_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CACertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.ca_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("WorkshopCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("WorkshopCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }
        // FIXME:
        Ok(Box::new(workshop_card))
    }
}
