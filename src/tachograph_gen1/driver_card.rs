use binary_data::BinSeek;
use log::{debug, trace};
use std::collections::HashMap;

use crate::{
    Readable, ReadableWithParams, Result,
    gen1::{CardResponseParameterData, CardVehicleRecord, Certificate, DriverCardApplicationIdentification, PlaceRecord},
    tacho::{
        Card, CardChipIdentification, CardControlActivityDataRecord, CardDriverActivity, CardDriverActivityParams,
        CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData,
        CardFileID, CardGeneration, CardIccIdentification, CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams,
        CardVehiclesUsed, CurrentUsage, Identification, IdentificationParams, SpecificConditions, SpecificConditionsParams,
        TimeReal, VehiclesUsedParams,
    },
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: DriverCardApplicationIdentification,
    pub card_download: Option<TimeReal>,
    pub card_event_data: Option<CardEventData>,
    pub card_driving_license_info: Option<CardDrivingLicenceInformation>,
    pub card_fault_data: Option<CardFaultData>,
    pub identification: Option<Identification>,
    pub card_driver_activity: Option<CardDriverActivity>,
    pub specific_conditions: Option<SpecificConditions>,
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    pub current_usage: Option<CurrentUsage>,
    pub card_vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub card_places: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    pub card_certificate: Option<Certificate>,
    pub ca_certificate: Option<Certificate>,
    pub card_notes: String,
}

impl DriverCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: DriverCardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen1,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            card_download: None,
            card_event_data: None,
            card_driving_license_info: None,
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

    pub fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<DriverCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            DriverCardApplicationIdentification,
        >(card_data_files)?;
        debug!("DriverCard::parse - Application Identification: {:?}", application_identification);

        let mut driver_card = DriverCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
        );

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - ID: {:?}", card_item.0);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            debug!("DriverCard::parse - ID: {:?}, Data Length: {:?}", card_item.0, reader.len()?);
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
                    driver_card.card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_faults_per_type,
                    );
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    driver_card.card_fault_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.card_activity_length_range,
                    );
                    let params = CardDriverActivityParams::new(application_identification.card_activity_length_range);
                    driver_card.card_driver_activity = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_of_card_vehicle_records,
                    );
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    driver_card.card_vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_of_card_place_records,
                    );
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 1);
                    driver_card.card_places = Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    driver_card.current_usage = Some(CurrentUsage::read(&mut reader)?);
                }
                CardFileID::ControlActivityData => {
                    driver_card.control_activity_data = Some(CardControlActivityDataRecord::read(&mut reader)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    driver_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::DrivingLicenseInfo => {
                    driver_card.card_driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
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
