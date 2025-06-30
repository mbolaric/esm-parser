use std::collections::HashMap;

use binary_data::BinSeek;
use log::{debug, trace};

use crate::{
    Readable, ReadableWithParams, Result,
    gen1::{CardResponseParameterData, DriverCardApplicationIdentification},
    tacho::{
        self, CardChipIdentification, CardControlActivityData, CardDataFile, CardDriverActivity, CardDriverActivityParams,
        CardDrivingLicenceInformation, CardEventData, CardEventDataParams, CardFaultData, CardFaultDataParams, CardFileID,
        CardIccIdentification, Identification, IdentificationParams, SpecificCondition, SpecificConditions,
        SpecificConditionsParams, TimeReal,
    },
};

#[derive(Debug)]
pub struct DriverCard {
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
    pub control_activity_data: Option<CardControlActivityData>,
}

impl DriverCard {
    fn parse_ic(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardChipIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::IC, card_data_files)?;
        let card_chip_identification = CardChipIdentification::read(&mut reader)?;
        Ok(card_chip_identification)
    }

    fn parse_icc(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardIccIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(&CardFileID::ICC, card_data_files)?;
        let card_icc_identification = CardIccIdentification::read(&mut reader)?;
        Ok(card_icc_identification)
    }

    fn parse_application_identification(
        card_data_files: &HashMap<CardFileID, CardDataFile>,
    ) -> Result<DriverCardApplicationIdentification> {
        let mut reader = <dyn tacho::Card<CardResponseParameterData>>::get_mem_reader(
            &CardFileID::ApplicationIdentification,
            card_data_files,
        )?;
        let application_identification = DriverCardApplicationIdentification::read(&mut reader)?;
        Ok(application_identification)
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<Box<DriverCard>> {
        let card_chip_identification = DriverCard::parse_ic(card_data_files)?;
        let card_icc_identification = DriverCard::parse_icc(card_data_files)?;
        let application_identification = DriverCard::parse_application_identification(card_data_files)?;
        debug!("DriverCard::parse - Application Identification: {:?}", application_identification);

        let mut card_download: Option<TimeReal> = None;
        let mut card_event_data: Option<CardEventData> = None;
        let mut card_driving_license_info: Option<CardDrivingLicenceInformation> = None;
        let mut card_fault_data: Option<CardFaultData> = None;
        let mut identification: Option<Identification> = None;
        let mut card_driver_activity: Option<CardDriverActivity> = None;
        let mut specific_conditions: Option<SpecificConditions> = None;
        let mut control_activity_data: Option<CardControlActivityData> = None;

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::CardDownload => {
                    card_download = Some(TimeReal::read(&mut reader)?);
                }
                CardFileID::EventsData => {
                    let params = CardEventDataParams::new(6, application_identification.no_events_per_type);
                    card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                }
                CardFileID::FaultsData => {
                    let params = CardFaultDataParams::new(application_identification.no_faults_per_type);
                    card_fault_data = Some(CardFaultData::read(&mut reader, &params)?);
                }
                CardFileID::DriverActivityData => {
                    let params = CardDriverActivityParams::new(application_identification.card_activity_length_range);
                    card_driver_activity = Some(CardDriverActivity::read(&mut reader, &params)?);
                }
                CardFileID::VehiclesUsed | CardFileID::Places | CardFileID::CurrentUsage => {
                    trace!("{:?} Not Implemented", card_item.0)
                }
                CardFileID::ControlActivityData => {
                    control_activity_data = Some(CardControlActivityData::read(&mut reader)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::DrivingLicenseInfo => {
                    card_driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
                }
                CardFileID::SpecificConditions => {
                    let params = SpecificConditionsParams::new(56);
                    specific_conditions = Some(SpecificConditions::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate | CardFileID::CACertificate => {
                    trace!("{:?} Not Implemented", card_item.0)
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("Already parsed: {:?}", card_item.0)
                }
                _ => trace!("Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(Self {
            card_chip_identification,
            card_icc_identification,
            application_identification,
            card_download,
            card_event_data,
            card_driving_license_info,
            card_fault_data,
            identification,
            card_driver_activity,
            specific_conditions,
            control_activity_data,
        }))
    }
}
