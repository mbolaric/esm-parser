use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{
        CardResponseParameterData, CardVehicleRecord, CardVehicleUnitsUsed, CardVehicleUnitsUsedParams, Certificate,
        CertificateParams, DriverCardApplicationIdentification, GnssAccumulatedDriving, GnssAccumulatedDrivingParams,
        PlaceRecord,
    },
    tacho::{
        Card, CardChipIdentification, CardControlActivityDataRecord, CardDriverActivity, CardDriverActivityParams,
        CardDrivingLicenceInformation, CardFaultData, CardFaultDataParams, CardFileData, CardFileID, CardGeneration,
        CardIccIdentification, CardPlaceDailyWorkPeriod, CardPlaceDailyWorkPeriodParams, CardVehiclesUsed, CurrentUsage,
        Identification, IdentificationParams, SpecificConditions, SpecificConditionsParams, TimeReal, VehiclesUsedParams,
    },
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: DriverCardApplicationIdentification,
    pub card_download: Option<TimeReal>,
    pub card_fault_data: Option<CardFaultData>,
    pub card_driver_activity: Option<CardDriverActivity>,
    pub card_vehicles_used: Option<CardVehiclesUsed<CardVehicleRecord>>,
    pub card_place_daily_work_period: Option<CardPlaceDailyWorkPeriod<PlaceRecord>>,
    pub current_usage: Option<CurrentUsage>,
    pub control_activity_data: Option<CardControlActivityDataRecord>,
    pub identification: Option<Identification>,
    pub card_driving_license_info: Option<CardDrivingLicenceInformation>,
    pub specific_conditions: Option<SpecificConditions>,
    pub card_vehicle_units_used: Option<CardVehicleUnitsUsed>,
    pub gnss_places: Option<GnssAccumulatedDriving>,
    pub card_certificate: Option<Certificate>,
    pub ca_certificate: Option<Certificate>,
    pub card_sign_certificate: Option<Certificate>,
    pub link_certificate: Option<Certificate>,
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
            card_fault_data: None,
            card_driver_activity: None,
            card_vehicles_used: None,
            card_place_daily_work_period: None,
            current_usage: None,
            control_activity_data: None,
            identification: None,
            card_driving_license_info: None,
            specific_conditions: None,
            card_vehicle_units_used: None,
            card_certificate: None,
            gnss_places: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
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
            debug!("DriverCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::CardDownload => {
                    driver_card.card_download = Some(TimeReal::read(&mut reader)?);
                }
                // FIXME: we need to parse all cases
                CardFileID::EventsData => {
                    trace!("DriverCard::parse - Not Implemented: {:?}", card_item.0)
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
                    let params = CardPlaceDailyWorkPeriodParams::new(application_identification.no_of_card_place_records, 2);
                    driver_card.card_place_daily_work_period =
                        Some(CardPlaceDailyWorkPeriod::<PlaceRecord>::read(&mut reader, &params)?);
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
                CardFileID::VehicleUnitsUsed => {
                    let params = CardVehicleUnitsUsedParams::new(application_identification.no_card_vehicle_units_records);
                    driver_card.card_vehicle_units_used = Some(CardVehicleUnitsUsed::read(&mut reader, &params)?);
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
