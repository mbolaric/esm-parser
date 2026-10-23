use std::collections::HashMap;

use binary_data::BinSeek;
use log::{debug, trace};
use serde::Serialize;

use crate::gen2::{
    BorderCrossings, BorderCrossingsParams, CardAuthenticationRecordsParams, CardPlaceAuthDailyWorkPeriod, CardVehicleRecord,
    CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, Certificate, CertificateParams, DriverCardApplicationIdentification,
    DriverCardApplicationIdentificationV2, GnssAccumulatedDriving, GnssAccumulatedDrivingParams, GnssAuthAccumulatedDriving,
    LoadTypeEntries, LoadTypeEntriesParams, LoadUnloadOperations, LoadUnloadOperationsParams, PlaceRecord, SpecificConditions,
    SpecificConditionsParams,
};
use crate::tacho::{
    CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity, CardDriverActivityParams,
    CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData,
    CardFileID, CardFilesDataByCardGenerationItem, CardGeneration, CardIccIdentification, CardParser, CardPlaceDailyWorkPeriod,
    CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, DataFiles, Identification, IdentificationParams, TimeReal,
    VehiclesUsedParams, sorted_card_files,
};
use crate::{Readable, ReadableWithParams, Result, ensure_consumed};

/// Driver card application generation 2
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2DriverCard"))]
pub struct DriverCard {
    #[serde(rename = "cardGeneration")]
    pub card_generation: CardGeneration,
    #[serde(rename = "cardChipIdentification")]
    pub card_chip_identification: CardChipIdentification,
    #[serde(rename = "cardIccIdentification")]
    pub card_icc_identification: CardIccIdentification,
    #[serde(rename = "applicationIdentification")]
    pub application_identification: DriverCardApplicationIdentification,
    #[serde(rename = "applicationIdentificationV2")]
    pub application_identification_v2: Option<DriverCardApplicationIdentificationV2>,
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
    #[serde(rename = "drivingLicenceInformation")]
    pub driving_license_info: Option<CardDrivingLicenceInformation>,
    #[serde(rename = "specificConditions")]
    pub specific_conditions: Option<SpecificConditions>,
    #[serde(rename = "vehicleUnitsUsed")]
    pub vehicle_units_used: Option<CardVehicleUnitsUsed>,
    #[serde(rename = "gnssPlaces")]
    pub gnss_places: Option<GnssAccumulatedDriving>,
    #[serde(rename = "placesAuthentication")]
    pub places_authentication: Option<CardPlaceAuthDailyWorkPeriod>,
    #[serde(rename = "gnssPlacesAuthentication")]
    pub gnss_places_authentication: Option<GnssAuthAccumulatedDriving>,
    #[serde(rename = "borderCrossings")]
    pub border_crossings: Option<BorderCrossings>,
    #[serde(rename = "loadUnloadOperations")]
    pub load_unload_operations: Option<LoadUnloadOperations>,
    #[serde(rename = "loadTypeEntries")]
    pub load_type_entries: Option<LoadTypeEntries>,
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
            card_generation: CardGeneration::Gen2,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            application_identification_v2: None,
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
            gnss_places: None,
            places_authentication: None,
            gnss_places_authentication: None,
            border_crossings: None,
            load_unload_operations: None,
            load_type_entries: None,
            card_certificate: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
            card_notes,
            data_files,
        }
    }
}

impl CardParser for DriverCard {
    fn parse(card_files: CardFilesDataByCardGenerationItem) -> Result<Box<DriverCard>> {
        let card_chip_identification = card_files.parse_required(&CardFileID::IC)?;
        let card_icc_identification = card_files.parse_required(&CardFileID::ICC)?;
        let application_identification =
            card_files.parse_required::<DriverCardApplicationIdentification>(&CardFileID::ApplicationIdentification)?;
        debug!("DriverCard::parse - Application Identification: {application_identification:?}");

        let application_identification_v2 =
            card_files.parse_optional::<DriverCardApplicationIdentificationV2>(&CardFileID::ApplicationIdentificationV2)?;

        let mut driver_card = DriverCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_files.card_notes,
            card_files.card_files_data,
        );

        driver_card.application_identification_v2 = application_identification_v2.clone();

        for card_item in sorted_card_files(&driver_card.data_files) {
            debug!("DriverCard::parse - ID: {:?}", card_item.0,);
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
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id);
                    driver_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::DrivingLicenseInfo => {
                    driver_card.driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(application_identification.no_of_specific_condition_records);
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
                CardFileID::PlacesAuthentication => {
                    let params = CardAuthenticationRecordsParams::new(application_identification.no_of_card_place_records);
                    driver_card.places_authentication = Some(CardPlaceAuthDailyWorkPeriod::read(&mut reader, &params)?);
                }
                CardFileID::GnssPlacesAuthentication => {
                    let params = CardAuthenticationRecordsParams::new(application_identification.no_gnssad_records);
                    driver_card.gnss_places_authentication = Some(GnssAuthAccumulatedDriving::read(&mut reader, &params)?);
                }
                CardFileID::BorderCrossings => {
                    let application_identification_v2 =
                        DriverCardApplicationIdentificationV2::require(application_identification_v2.as_ref(), card_item.0)?;
                    let params = BorderCrossingsParams::new(application_identification_v2.no_of_border_crossing_records);
                    driver_card.border_crossings = Some(BorderCrossings::read(&mut reader, &params)?);
                }
                CardFileID::LoadUnloadOperations => {
                    let application_identification_v2 =
                        DriverCardApplicationIdentificationV2::require(application_identification_v2.as_ref(), card_item.0)?;
                    let params = LoadUnloadOperationsParams::new(application_identification_v2.no_of_load_unload_records);
                    driver_card.load_unload_operations = Some(LoadUnloadOperations::read(&mut reader, &params)?);
                }
                CardFileID::LoadTypeEntries => {
                    let application_identification_v2 =
                        DriverCardApplicationIdentificationV2::require(application_identification_v2.as_ref(), card_item.0)?;
                    let params = LoadTypeEntriesParams::new(application_identification_v2.no_of_load_type_entry_records);
                    driver_card.load_type_entries = Some(LoadTypeEntries::read(&mut reader, &params)?);
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
                CardFileID::IC
                | CardFileID::ICC
                | CardFileID::ApplicationIdentification
                | CardFileID::ApplicationIdentificationV2 => {
                    trace!("DriverCard::parse - Already parsed: {:?}", card_item.0);
                    continue;
                }
                _ => {
                    trace!("DriverCard::parse - Not Parsed: {:?}", card_item.0);
                    continue;
                }
            }
            ensure_consumed(&mut reader, &card_file.card_file_id.to_string())?;
        }

        Ok(Box::new(driver_card))
    }
}

impl DataFiles for DriverCard {
    fn get_data_files(&self) -> &crate::tacho::CardFilesMap {
        &self.data_files
    }
}
