use std::collections::HashMap;

use log::{debug, trace};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{
        CardResponseParameterData, CardVehicleRecord, CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, Certificate,
        CertificateParams, GnssAccumulatedDriving, GnssAccumulatedDrivingParams, PlaceRecord,
        WorkshopCardApplicationIdentification, WorkshopCardCalibrationRecord,
    },
    tacho::{
        Card, CardChipIdentification, CardCurrentUse, CardDriverActivity, CardDriverActivityParams, CardEventData,
        CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData, CardFileID, CardGeneration, CardIccIdentification,
        CardParser, CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, Identification,
        IdentificationParams, SpecificConditions, SpecificConditionsParams, VehiclesUsedParams, WorkshopCardCalibrationData,
        WorkshopCardCalibrationDataParams,
    },
};

#[derive(Debug, Serialize)]
pub struct WorkshopCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: WorkshopCardApplicationIdentification,
    pub card_event_data: Option<CardEventData>,
    pub card_fault_data: Option<CardFaultData>,
    pub current_usage: Option<CardCurrentUse>,
    pub card_vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub card_place_daily_work_period: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    pub specific_conditions: Option<SpecificConditions>,
    pub card_driver_activity: Option<CardDriverActivity>,
    pub card_calibration_data: Option<WorkshopCardCalibrationData<WorkshopCardCalibrationRecord>>,
    pub card_vehicle_units_used: Option<CardVehicleUnitsUsed>,
    pub identification: Option<Identification>,
    pub gnss_places: Option<GnssAccumulatedDriving>,
    pub card_certificate: Option<Certificate>,
    pub ca_certificate: Option<Certificate>,
    pub card_sign_certificate: Option<Certificate>,
    pub link_certificate: Option<Certificate>,
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
            card_event_data: None,
            card_fault_data: None,
            current_usage: None,
            card_vehicles_used: None,
            card_place_daily_work_period: None,
            specific_conditions: None,
            card_driver_activity: None,
            card_calibration_data: None,
            card_vehicle_units_used: None,
            identification: None,
            gnss_places: None,
            card_certificate: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
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
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(11, application_identification.no_events_per_type);
                    workshop_card.card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    workshop_card.card_fault_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    workshop_card.card_vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 2);
                    workshop_card.card_place_daily_work_period =
                        Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    workshop_card.current_usage = Some(CardCurrentUse::read(&mut reader)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.card_activity_length_range);
                    workshop_card.card_driver_activity = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    workshop_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(application_identification.no_of_specific_condition_records as u8);
                    workshop_card.specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
                CardFileID::VehicleUnitsUsed => {
                    let params = CardVehicleUnitsUsedParams::new(application_identification.no_of_card_vehicle_unit_records);
                    workshop_card.card_vehicle_units_used = Some(CardVehicleUnitsUsed::read(&mut reader, &params)?);
                }
                CardFileID::GnssPlaces => {
                    let params = GnssAccumulatedDrivingParams::new(application_identification.no_of_gnssad_records);
                    workshop_card.gnss_places = Some(GnssAccumulatedDriving::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.card_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CACertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.ca_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CardSignCertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.card_sign_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::LinkCertificate => {
                    let params = CertificateParams::new(None);
                    workshop_card.link_certificate = Some(Certificate::read(&mut reader, &params)?);
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
