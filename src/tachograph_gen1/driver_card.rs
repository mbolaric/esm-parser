use log::{debug, trace};
use std::collections::HashMap;

use crate::{
    Readable, ReadableWithParams, Result,
    gen1::{CardApplicationIdentification, CardData, CardResponseParameterData, PlaceRecord, VehiclesUsedRecord},
    tacho::{
        CardChipIdentification, CardControlActivityData, CardDataFile, CardDriverActivity, CardDriverActivityParams,
        CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileID,
        CardIccIdentification, CardPlaces, CardPlacesParams, CurrentUsage, Identification, IdentificationParams,
        SpecificConditions, SpecificConditionsParams, TimeReal, VehiclesUsed, VehiclesUsedParams,
    },
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: CardApplicationIdentification,
    pub card_download: Option<TimeReal>,
    pub card_event_data: Option<CardEventData>,
    pub card_driving_license_info: Option<CardDrivingLicenceInformation>,
    pub card_fault_data: Option<CardFaultData>,
    pub identification: Option<Identification>,
    pub card_driver_activity: Option<CardDriverActivity>,
    pub specific_conditions: Option<SpecificConditions>,
    pub control_activity_data: Option<CardControlActivityData>,
    pub current_usage: Option<CurrentUsage>,
    pub vehicles_used: Option<VehiclesUsed<VehiclesUsedRecord>>,
    pub card_places: Option<CardPlaces<PlaceRecord>>,
    pub card_notes: String,
}

impl DriverCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: CardApplicationIdentification,
        card_notes: String,
    ) -> Self {
        Self {
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
            vehicles_used: None,
            card_places: None,
            card_notes,
        }
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &String) -> Result<Box<DriverCard>> {
        let card_chip_identification = CardData::parse_ic(card_data_files)?;
        let card_icc_identification = CardData::parse_icc(card_data_files)?;
        let application_identification = CardData::parse_card_application_identification(card_data_files)?;
        debug!("DriverCard::parse - Application Identification: {:?}", application_identification);

        let mut driver_card = DriverCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.clone(),
        );

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::CardDownload => {
                    driver_card.card_download = Some(TimeReal::read(&mut reader)?);
                }
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(6, application_identification.no_events_per_type);
                    driver_card.card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    driver_card.card_fault_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.card_activity_length_range);
                    driver_card.card_driver_activity = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    driver_card.vehicles_used = Some(VehiclesUsed::<VehiclesUsedRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    let params = CardPlacesParams::new(application_identification.no_of_place_records, 1);
                    driver_card.card_places = Some(CardPlaces::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    driver_card.current_usage = Some(CurrentUsage::read(&mut reader)?);
                }
                CardFileID::ControlActivityData => {
                    driver_card.control_activity_data = Some(CardControlActivityData::read(&mut reader)?);
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
                    trace!("{:?} Not Implemented", card_item.0)
                }
                CardFileID::CACertificate => {
                    trace!("{:?} Not Implemented", card_item.0)
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("Already parsed: {:?}", card_item.0)
                }
                _ => trace!("Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(driver_card))
    }
}
