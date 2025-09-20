use std::collections::HashMap;

use log::{debug, trace};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{
        CardResponseParameterData, CardVehicleRecord, CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, Certificate,
        CertificateParams, DriverCardApplicationIdentification, GnssAccumulatedDriving, GnssAccumulatedDrivingParams,
        PlaceRecord, SpecificConditions, SpecificConditionsParams,
    },
    tacho::{
        Card, CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity,
        CardDriverActivityParams, CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData,
        CardFaultDataParams, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser,
        CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, Identification, IdentificationParams,
        TimeReal, VehiclesUsedParams,
    },
};

/// Driver card application generation 2
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
    #[serde(rename = "faultsData")]
    pub faults_data: Option<CardFaultData>,
    #[serde(rename = "driverActivityData")]
    pub driver_activity_data: Option<CardDriverActivity>,
    #[serde(rename = "vehiclesUsed")]
    pub vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub places: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    #[serde(rename = "currentUsage")]
    pub current_usage: Option<CardCurrentUse>,
    #[serde(rename = "controlActivityData")]
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    pub identification: Option<Identification>,
    #[serde(rename = "drivingLicenceInfo")]
    pub driving_license_info: Option<CardDrivingLicenceInformation>,
    #[serde(rename = "specificConditions")]
    pub specific_conditions: Option<SpecificConditions>,
    #[serde(rename = "vehicleUnitsUsed")]
    pub vehicle_units_used: Option<CardVehicleUnitsUsed>,
    #[serde(rename = "gnssPlaces")]
    pub gnss_places: Option<GnssAccumulatedDriving>,
    #[serde(rename = "cardCertificate")]
    pub card_certificate: Option<Certificate>,
    #[serde(rename = "caCertificate")]
    pub ca_certificate: Option<Certificate>,
    #[serde(rename = "cardSignCertificate")]
    pub card_sign_certificate: Option<Certificate>,
    #[serde(rename = "linkCertificate")]
    pub link_certificate: Option<Certificate>,
    #[serde(rename = "cardNotes")]
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
            card_generation: CardGeneration::Gen2,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            card_download: None,
            events_data: None,
            faults_data: None,
            driver_activity_data: None,
            vehicles_used: None,
            places: None,
            current_usage: None,
            control_activity_data: None,
            identification: None,
            driving_license_info: None,
            specific_conditions: None,
            vehicle_units_used: None,
            card_certificate: None,
            gnss_places: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
            card_notes,
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
                    debug!(
                        "DriverCard::parse - ID: {:?}, Number Of Records: {:?}",
                        card_item.0, application_identification.no_events_per_type,
                    );
                    let params = CardEventDataParams::new(11, application_identification.no_events_per_type);
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
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 2);
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
                    let params = SpecificConditionsParams::new(application_identification.no_of_specific_condition_records as u8);
                    driver_card.specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
                CardFileID::VehicleUnitsUsed => {
                    let params = CardVehicleUnitsUsedParams::new(application_identification.no_card_vehicle_units_records);
                    driver_card.vehicle_units_used = Some(CardVehicleUnitsUsed::read(&mut reader, &params)?);
                }
                CardFileID::GnssPlaces => {
                    let params = GnssAccumulatedDrivingParams::new(application_identification.no_gnssad_records);
                    driver_card.gnss_places = Some(GnssAccumulatedDriving::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    let params = CertificateParams::new(None);
                    driver_card.card_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CACertificate => {
                    let params = CertificateParams::new(None);
                    driver_card.ca_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CardSignCertificate => {
                    let params = CertificateParams::new(None);
                    driver_card.card_sign_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::LinkCertificate => {
                    let params = CertificateParams::new(None);
                    driver_card.link_certificate = Some(Certificate::read(&mut reader, &params)?);
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
