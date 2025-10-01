use std::collections::HashMap;

use binary_data::{BigEndian, BinSeek, ReadBytes};
use log::{debug, trace};
use serde::Serialize;

use crate::gen1::{
    CardResponseParameterData, CardVehicleRecord, Certificate, PlaceRecord, SpecificConditions, SpecificConditionsParams,
    WorkshopCardApplicationIdentification, WorkshopCardCalibrationRecord,
};
use crate::tacho::{
    Card, CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity, CardDriverActivityParams,
    CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData, CardFileID, CardIccIdentification,
    CardParser, CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, Identification, IdentificationParams,
    VehiclesUsedParams, WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams,
};
use crate::{Readable, ReadableWithParams, Result};

/// Workshop card application generation 1
#[derive(Debug, Serialize)]
pub struct WorkshopCard {
    #[serde(rename = "cardChipIdentification")]
    pub card_chip_identification: CardChipIdentification,
    #[serde(rename = "cardIccIdentification")]
    pub card_icc_identification: CardIccIdentification,
    #[serde(rename = "applicationIdentification")]
    pub application_identification: WorkshopCardApplicationIdentification,
    #[serde(rename = "noOfCalibrationsSinceDownload")]
    pub no_of_calibrations_since_download: u16,
    pub calibration: Option<WorkshopCardCalibrationData<WorkshopCardCalibrationRecord>>,
    #[serde(rename = "eventsData")]
    pub events_data: Option<CardEventData>,
    #[serde(rename = "faultsData")]
    pub faults_data: Option<CardFaultData>,
    pub identification: Option<Identification>,
    #[serde(rename = "driverActivityData")]
    pub driver_activity_data: Option<CardDriverActivity>,
    #[serde(rename = "specificConditions")]
    pub specific_conditions: Option<SpecificConditions>,
    #[serde(rename = "controlActivityData")]
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    #[serde(rename = "currentUsage")]
    pub current_usage: Option<CardCurrentUse>,
    #[serde(rename = "vehiclesUsed")]
    pub vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub places: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    #[serde(rename = "cardCertificate")]
    pub card_certificate: Option<Certificate>,
    #[serde(rename = "caCertificate")]
    pub ca_certificate: Option<Certificate>,
    #[serde(rename = "cardNotes")]
    pub card_notes: String,
    #[serde(rename = "dataFiles")]
    pub data_files: HashMap<CardFileID, CardFileData>,
}

impl WorkshopCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: WorkshopCardApplicationIdentification,
        card_notes: String,
        data_files: HashMap<CardFileID, CardFileData>,
    ) -> Self {
        Self {
            card_chip_identification,
            card_icc_identification,
            application_identification,
            no_of_calibrations_since_download: 0,
            calibration: None,
            events_data: None,
            faults_data: None,
            identification: None,
            driver_activity_data: None,
            specific_conditions: None,
            control_activity_data: None,
            current_usage: None,
            vehicles_used: None,
            places: None,
            card_certificate: None,
            ca_certificate: None,
            card_notes,
            data_files,
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
        debug!("WorkshopCard::parse - Application Identification: {application_identification:?}");

        let mut workshop_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
            (*card_data_files).clone(),
        );

        for card_item in card_data_files.iter() {
            debug!("WorkshopCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            debug!(
                "WorkshopCard::parse - ID: {:?}, Data Length: {:?}, Has Signature: {}",
                card_item.0,
                reader.len()?,
                card_file.signature.is_some()
            );
            match card_item.0 {
                CardFileID::CardDownload => {
                    workshop_card.no_of_calibrations_since_download = reader.read_u16::<BigEndian>()?;
                }
                CardFileID::Calibration => {
                    let params = WorkshopCardCalibrationDataParams::new(application_identification.no_off_calibration_records);
                    workshop_card.calibration =
                        Some(WorkshopCardCalibrationData::<WorkshopCardCalibrationRecord>::read(&mut reader, &params)?);
                }
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(6, application_identification.no_events_per_type);
                    workshop_card.events_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    workshop_card.faults_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.activity_structure_length);
                    workshop_card.driver_activity_data = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    workshop_card.vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 1);
                    workshop_card.places = Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    workshop_card.current_usage = Some(CardCurrentUse::read(&mut reader)?);
                }
                CardFileID::ControlActivityData => {
                    workshop_card.control_activity_data = Some(CardControlActivityDataRecord::read(&mut reader)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(56);
                    workshop_card.specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
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
