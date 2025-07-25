use std::collections::HashMap;

use binary_data::{BigEndian, ReadBytes};
use log::{debug, trace};

use crate::gen1::{
    CardResponseParameterData, CardVehicleRecord, Certificate, PlaceRecord, WorkshopCardApplicationIdentification,
    WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams,
};
use crate::tacho::{
    Card, CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity, CardDriverActivityParams,
    CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData, CardFileID, CardIccIdentification,
    CardParser, CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, Identification, IdentificationParams,
    SpecificConditions, SpecificConditionsParams, VehiclesUsedParams,
};
use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug)]
pub struct WorkshopCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: WorkshopCardApplicationIdentification,

    pub no_of_calibrations_since_download: u16,
    pub card_calibration_data: Option<WorkshopCardCalibrationData>,
    pub card_event_data: Option<CardEventData>,
    pub card_fault_data: Option<CardFaultData>,
    pub identification: Option<Identification>,
    pub card_driver_activity: Option<CardDriverActivity>,
    pub specific_conditions: Option<SpecificConditions>,
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    pub current_usage: Option<CardCurrentUse>,
    pub card_vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub card_places: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
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
            card_event_data: None,
            card_fault_data: None,
            identification: None,
            card_driver_activity: None,
            specific_conditions: None,
            control_activity_data: None,
            current_usage: None,
            card_vehicles_used: None,
            card_places: None,
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
        debug!("WorkshopCard::parse - Application Identification: {:?}", application_identification);

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
                CardFileID::CardDownload => {
                    workshop_card.no_of_calibrations_since_download = reader.read_u16::<BigEndian>()?;
                }
                CardFileID::Calibration => {
                    let params = WorkshopCardCalibrationDataParams::new(application_identification.no_off_calibration_records);
                    workshop_card.card_calibration_data = Some(WorkshopCardCalibrationData::read(&mut reader, &params)?);
                }
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(6, application_identification.no_events_per_type);
                    workshop_card.card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    workshop_card.card_fault_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.card_activity_length_range);
                    workshop_card.card_driver_activity = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    workshop_card.card_vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 1);
                    workshop_card.card_places = Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
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
