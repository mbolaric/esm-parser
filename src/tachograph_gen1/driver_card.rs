use binary_data::BinSeek;
use log::{debug, trace};
use serde::Serialize;
use std::collections::HashMap;

use crate::{
    Readable, ReadableWithParams, Result,
    gen1::{
        CardResponseParameterData, CardVehicleRecord, Certificate, DriverCardApplicationIdentification, PlaceRecord,
        SpecificConditions, SpecificConditionsParams,
    },
    tacho::{
        Card, CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity,
        CardDriverActivityParams, CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData,
        CardFaultDataParams, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser,
        CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, DataFiles, Identification,
        IdentificationParams, TimeReal, VehiclesUsedParams,
    },
};

/// Driver Card application generation 1
#[derive(Debug, Serialize)]
pub struct DriverCard {
    #[serde(rename = "cardGeneration")]
    pub card_generation: CardGeneration,
    #[serde(rename = "cardChipIdentification")]
    pub card_chip_identification: CardChipIdentification,
    #[serde(rename = "cardIccIdentification")]
    pub card_icc_identification: CardIccIdentification,
    #[serde(rename = "applicationIdentification")]
    pub application_identification: DriverCardApplicationIdentification,
    #[serde(rename = "cardDownload")]
    pub card_download: Option<TimeReal>,
    #[serde(rename = "eventsData")]
    pub events_data: Option<CardEventData>,
    #[serde(rename = "drivingLicenceInformation")]
    pub driving_license_info: Option<CardDrivingLicenceInformation>,
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

impl DriverCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: DriverCardApplicationIdentification,
        card_notes: String,
        data_files: HashMap<CardFileID, CardFileData>,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen1,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            card_download: None,
            events_data: None,
            driving_license_info: None,
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

impl CardParser<DriverCard> for DriverCard {
    fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<DriverCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            DriverCardApplicationIdentification,
        >(card_data_files)?;
        debug!("DriverCard::parse - Application Identification: {application_identification:?}");

        let mut driver_card = DriverCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
            (*card_data_files).clone(),
        );
        debug!("DriverCard::parse - CARD: {driver_card:?}");

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - ID: {:?}", card_item.0);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            debug!(
                "DriverCard::parse - ID: {:?}, Data Length: {:?}, Has Signature: {}",
                card_item.0,
                reader.len()?,
                card_file.signature.is_some()
            );
            match card_item.0 {
                CardFileID::CardDownload => {
                    driver_card.card_download = Some(TimeReal::read(&mut reader)?);
                }
                CardFileID::EventsData => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_events_per_type,
                    );
                    let params = CardEventDataParams::new(6, application_identification.no_events_per_type);
                    driver_card.events_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_faults_per_type,
                    );
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    driver_card.faults_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.activity_structure_length,
                    );
                    let params = CardDriverActivityParams::new(application_identification.activity_structure_length);
                    driver_card.driver_activity_data = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_of_card_vehicle_records,
                    );
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    driver_card.vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_of_card_place_records,
                    );
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 1);
                    driver_card.places = Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    driver_card.current_usage = Some(CardCurrentUse::read(&mut reader)?);
                }
                CardFileID::ControlActivityData => {
                    driver_card.control_activity_data = Some(CardControlActivityDataRecord::read(&mut reader)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    driver_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::DrivingLicenseInfo => {
                    driver_card.driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(56);
                    driver_card.specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    driver_card.card_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::CACertificate => {
                    driver_card.ca_certificate = Some(Certificate::read(&mut reader)?);
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("DriverCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("DriverCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(driver_card))
    }
}

impl DataFiles for DriverCard {
    fn get_data_files(&self) -> &crate::tacho::CardFilesMap {
        &self.data_files
    }
}
