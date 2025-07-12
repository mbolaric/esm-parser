use std::collections::HashMap;

use binary_data::{BigEndian, ReadBytes};
use log::{debug, trace};

use crate::gen1::{
    CardData, Certificate, WorkshopCardApplicationIdentification, WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams,
};
use crate::tacho::{
    CardChipIdentification, CardDataFile, CardFileID, CardIccIdentification, Identification, IdentificationParams,
};
use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug)]
pub struct WorkshopCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: WorkshopCardApplicationIdentification,

    pub no_of_calibrations_since_download: u16,
    pub card_calibration_data: Option<WorkshopCardCalibrationData>,
    // pub card_event_data: Option<CardEventData>,
    // pub card_fault_data: Option<CardFaultData>,
    pub identification: Option<Identification>,
    // pub card_driver_activity: Option<CardDriverActivity>,
    // pub specific_conditions: Option<SpecificConditions>,
    // pub control_activity_data: Option<CardControlActivityData>,
    // pub current_usage: Option<CurrentUsage>,
    // pub vehicles_used: Option<VehiclesUsed<VehiclesUsedRecord>>,
    // pub card_places: Option<CardPlaces<PlaceRecord>>,
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
            card_chip_identification,
            card_icc_identification,
            application_identification,
            no_of_calibrations_since_download: 0,
            card_calibration_data: None,
            // card_event_data: None,
            // card_fault_data: None,
            identification: None,
            // card_driver_activity: None,
            // specific_conditions: None,
            // control_activity_data: None,
            // current_usage: None,
            // vehicles_used: None,
            // card_places: None,
            card_certificate: None,
            ca_certificate: None,
            card_notes,
        }
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String) -> Result<Box<WorkshopCard>> {
        let card_chip_identification = CardData::parse_ic(card_data_files)?;
        let card_icc_identification = CardData::parse_icc(card_data_files)?;
        let application_identification =
            CardData::parse_card_application_identification::<WorkshopCardApplicationIdentification>(card_data_files)?;
        debug!("WorkshopCard::parse - Application Identification: {:?}", application_identification);

        let mut workshop_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.clone(),
        );

        for card_item in card_data_files.iter() {
            debug!("WorkshopCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::CardDownload => {
                    workshop_card.no_of_calibrations_since_download = reader.read_u16::<BigEndian>()?;
                }
                CardFileID::Calibration => {
                    let params = WorkshopCardCalibrationDataParams::new(application_identification.no_off_calibration_records);
                    workshop_card.card_calibration_data = Some(WorkshopCardCalibrationData::read(&mut reader, &params)?);
                }
                CardFileID::EventsData => {}
                CardFileID::FaultsData => {}
                CardFileID::DriverActivityData => {}
                CardFileID::VehiclesUsed => {}
                CardFileID::Places => {}
                CardFileID::CurrentUsage => {}
                CardFileID::ControlActivityData => {}
                CardFileID::SpecificConditions => {}
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    workshop_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    workshop_card.card_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::CACertificate => {
                    workshop_card.ca_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("WorkshopCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("WorkshopCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(workshop_card))
    }
}
