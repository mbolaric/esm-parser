use std::collections::HashMap;

use binary_data::BinSeek;
use log::{debug, trace};
use serde::Serialize;

use crate::gen2::{
    BorderCrossings, BorderCrossingsParams, CardAuthenticationRecordsParams, CardPlaceAuthDailyWorkPeriod, CardVehicleRecord,
    CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, Certificate, CertificateParams, GnssAccumulatedDriving,
    GnssAccumulatedDrivingParams, GnssAuthAccumulatedDriving, LoadTypeEntries, LoadTypeEntriesParams, LoadUnloadOperations,
    LoadUnloadOperationsParams, PlaceRecord, SpecificConditions, SpecificConditionsParams, WorkshopCardApplicationIdentification,
    WorkshopCardApplicationIdentificationV2, WorkshopCardCalibrationAddData, WorkshopCardCalibrationAddDataParams,
    WorkshopCardCalibrationRecord,
};
use crate::tacho::{
    CardChipIdentification, CardControlActivityDataRecord, CardCurrentUse, CardDriverActivity, CardDriverActivityParams,
    CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileData, CardFileID,
    CardFilesDataByCardGenerationItem, CardGeneration, CardIccIdentification, CardParser, CardPlaceDailyWorkPeriod,
    CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, DataFiles, Identification, IdentificationParams, VehiclesUsedParams,
    WorkshopCardCalibrationData, WorkshopCardCalibrationDataParams, sorted_card_files,
};
use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2WorkshopCard"))]
pub struct WorkshopCard {
    #[serde(rename = "cardGeneration")]
    pub card_generation: CardGeneration,
    #[serde(rename = "cardChipIdentification")]
    pub card_chip_identification: CardChipIdentification,
    #[serde(rename = "cardIccIdentification")]
    pub card_icc_identification: CardIccIdentification,
    #[serde(rename = "applicationIdentification")]
    pub application_identification: WorkshopCardApplicationIdentification,
    #[serde(rename = "applicationIdentificationV2")]
    pub application_identification_v2: Option<WorkshopCardApplicationIdentificationV2>,
    #[serde(rename = "eventsData")]
    pub events_data: Option<CardEventData>,
    #[serde(rename = "faultsData")]
    pub faults_data: Option<CardFaultData>,
    #[serde(rename = "currentUsage")]
    pub current_usage: Option<CardCurrentUse>,
    #[serde(rename = "controlActivityData")]
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    #[serde(rename = "vehiclesUsed")]
    pub vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub places: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    #[serde(rename = "specificConditions")]
    pub specific_conditions: Option<SpecificConditions>,
    #[serde(rename = "driverActivityData")]
    pub driver_activity_data: Option<CardDriverActivity>,
    pub calibration: Option<WorkshopCardCalibrationData<WorkshopCardCalibrationRecord>>,
    /// Match to calibration records by oldTimeValue and VIN; physical indices and pointers are independent.
    #[serde(rename = "calibrationAddData")]
    pub calibration_add_data: Option<WorkshopCardCalibrationAddData>,
    #[serde(rename = "vehicleUnitsUsed")]
    pub vehicle_units_used: Option<CardVehicleUnitsUsed>,
    pub identification: Option<Identification>,
    #[serde(rename = "gnssPlaces")]
    pub gnss_places: Option<GnssAccumulatedDriving>,
    #[serde(rename = "placesAuthentication")]
    pub places_authentication: Option<CardPlaceAuthDailyWorkPeriod>,
    #[serde(rename = "gnssPlacesAuthentication")]
    pub gnss_places_authentication: Option<GnssAuthAccumulatedDriving>,
    #[serde(rename = "loadTypeEntries")]
    pub load_type_entries: Option<LoadTypeEntries>,
    #[serde(rename = "borderCrossings")]
    pub border_crossings: Option<BorderCrossings>,
    #[serde(rename = "loadUnloadOperations")]
    pub load_unload_operations: Option<LoadUnloadOperations>,
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

impl WorkshopCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: WorkshopCardApplicationIdentification,
        card_notes: String,
        data_files: HashMap<CardFileID, CardFileData>,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen2,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            application_identification_v2: None,
            events_data: None,
            faults_data: None,
            current_usage: None,
            control_activity_data: None,
            vehicles_used: None,
            places: None,
            specific_conditions: None,
            driver_activity_data: None,
            calibration: None,
            calibration_add_data: None,
            vehicle_units_used: None,
            identification: None,
            gnss_places: None,
            places_authentication: None,
            gnss_places_authentication: None,
            load_type_entries: None,
            border_crossings: None,
            load_unload_operations: None,
            card_certificate: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
            card_notes,
            data_files,
        }
    }
}

impl CardParser for WorkshopCard {
    fn parse(card_files: CardFilesDataByCardGenerationItem) -> Result<Box<WorkshopCard>> {
        let card_chip_identification = card_files.parse_required(&CardFileID::IC)?;
        let card_icc_identification = card_files.parse_required(&CardFileID::ICC)?;
        let application_identification =
            card_files.parse_required::<WorkshopCardApplicationIdentification>(&CardFileID::ApplicationIdentification)?;
        let application_identification_v2 =
            card_files.parse_optional::<WorkshopCardApplicationIdentificationV2>(&CardFileID::ApplicationIdentificationV2)?;

        let mut workshop_card = WorkshopCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_files.card_notes,
            card_files.card_files_data,
        );
        workshop_card.application_identification_v2 = application_identification_v2;

        for card_item in sorted_card_files(&workshop_card.data_files) {
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
                CardFileID::Calibration => {
                    let params = WorkshopCardCalibrationDataParams::new(
                        application_identification.no_off_calibration_records,
                        CardGeneration::Gen2,
                    );
                    workshop_card.calibration =
                        Some(WorkshopCardCalibrationData::<WorkshopCardCalibrationRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CalibrationAddData => {
                    let params = WorkshopCardCalibrationAddDataParams::new(application_identification.no_off_calibration_records);
                    workshop_card.calibration_add_data = Some(WorkshopCardCalibrationAddData::read(&mut reader, &params)?);
                }
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(11, application_identification.no_events_per_type);
                    workshop_card.events_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    workshop_card.faults_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed => {
                    let params = VehiclesUsedParams::new(application_identification.no_of_card_vehicle_records);
                    workshop_card.vehicles_used = Some(CardVehiclesUsed::<CardVehicleRecord>::read(&mut reader, &params)?);
                }
                CardFileID::Places => {
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 2);
                    workshop_card.places = Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
                }
                CardFileID::CurrentUsage => {
                    workshop_card.current_usage = Some(CardCurrentUse::read(&mut reader)?);
                }
                CardFileID::ControlActivityData => {
                    workshop_card.control_activity_data = Some(CardControlActivityDataRecord::read(&mut reader)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.activity_structure_length);
                    workshop_card.driver_activity_data = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id);
                    workshop_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(application_identification.no_of_specific_condition_records as u8);
                    workshop_card.specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
                CardFileID::VehicleUnitsUsed => {
                    let params = CardVehicleUnitsUsedParams::new(application_identification.no_of_card_vehicle_unit_records);
                    workshop_card.vehicle_units_used = Some(CardVehicleUnitsUsed::read(&mut reader, &params)?);
                }
                CardFileID::GnssPlaces => {
                    let params = GnssAccumulatedDrivingParams::new(application_identification.no_of_gnssad_records);
                    workshop_card.gnss_places = Some(GnssAccumulatedDriving::read(&mut reader, &params)?);
                }
                CardFileID::PlacesAuthentication => {
                    let params = CardAuthenticationRecordsParams::new(application_identification.no_of_card_place_records);
                    workshop_card.places_authentication = Some(CardPlaceAuthDailyWorkPeriod::read(&mut reader, &params)?);
                }
                CardFileID::GnssPlacesAuthentication => {
                    let params = CardAuthenticationRecordsParams::new(application_identification.no_of_gnssad_records);
                    workshop_card.gnss_places_authentication = Some(GnssAuthAccumulatedDriving::read(&mut reader, &params)?);
                }
                CardFileID::BorderCrossings => {
                    let application_identification_v2 = WorkshopCardApplicationIdentificationV2::require(
                        workshop_card.application_identification_v2.as_ref(),
                        card_item.0,
                    )?;
                    let params = BorderCrossingsParams::new(application_identification_v2.no_of_border_crossing_records);
                    workshop_card.border_crossings = Some(BorderCrossings::read(&mut reader, &params)?);
                }
                CardFileID::LoadUnloadOperations => {
                    let application_identification_v2 = WorkshopCardApplicationIdentificationV2::require(
                        workshop_card.application_identification_v2.as_ref(),
                        card_item.0,
                    )?;
                    let params = LoadUnloadOperationsParams::new(application_identification_v2.no_of_load_unload_records);
                    workshop_card.load_unload_operations = Some(LoadUnloadOperations::read(&mut reader, &params)?);
                }
                CardFileID::LoadTypeEntries => {
                    let application_identification_v2 = WorkshopCardApplicationIdentificationV2::require(
                        workshop_card.application_identification_v2.as_ref(),
                        card_item.0,
                    )?;
                    let params = LoadTypeEntriesParams::new(application_identification_v2.no_of_load_type_entry_records);
                    workshop_card.load_type_entries = Some(LoadTypeEntries::read(&mut reader, &params)?);
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
                CardFileID::IC
                | CardFileID::ICC
                | CardFileID::ApplicationIdentification
                | CardFileID::ApplicationIdentificationV2 => {
                    trace!("WorkshopCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("WorkshopCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }
        Ok(Box::new(workshop_card))
    }
}

impl DataFiles for WorkshopCard {
    fn get_data_files(&self) -> &crate::tacho::CardFilesMap {
        &self.data_files
    }
}
