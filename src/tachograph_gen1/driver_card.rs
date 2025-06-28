use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    Readable, ReadableWithParams, Result,
    gen1::{CardResponseParameterData, DriverCardApplicationIdentification},
    tacho::{
        self, CardChipIdentification, CardDataFile, CardDrivingLicenceInformation, CardEventData, CardEventDataParams,
        CardFaultData, CardFaultDataParams, CardFileID, CardIccIdentification, Identification, IdentificationParams, TimeReal,
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
                CardFileID::DriverActivityData
                | CardFileID::VehiclesUsed
                | CardFileID::Places
                | CardFileID::CurrentUsage
                | CardFileID::ControlActivityData => {
                    debug!("Not Implemented")
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::DrivingLicenseInfo => {
                    card_driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
                }
                CardFileID::SpecificConditions | CardFileID::CardCertificate | CardFileID::CACertificate => {
                    debug!("Not Implemented")
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
        }))
    }
}
