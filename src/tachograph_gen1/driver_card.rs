use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    Error, Readable, ReadableWithParams, Result,
    gen1::{CardResponseParameterData, DriverCardApplicationIdentification},
    tacho::{
        self, CardChipIdentification, CardDataFile, CardDrivingLicenceInformation, CardEventData, CardEventDataParams,
        CardFileID, CardIccIdentification, TimeReal,
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

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<Box<DriverCard>> {
        let card_chip_identification = DriverCard::parse_ic(card_data_files)?;
        let card_icc_identification = DriverCard::parse_icc(card_data_files)?;

        let mut application_identification: Option<DriverCardApplicationIdentification> = None;
        let mut card_download: Option<TimeReal> = None;
        let mut card_event_data: Option<CardEventData> = None;
        let mut card_driving_license_info: Option<CardDrivingLicenceInformation> = None;

        for card_item in card_data_files.iter() {
            debug!("DriverCard::parse - {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            match card_item.0 {
                CardFileID::ApplicationIdentification => {
                    application_identification = Some(DriverCardApplicationIdentification::read(&mut reader)?);
                }
                CardFileID::CardDownload => {
                    card_download = Some(TimeReal::read(&mut reader)?);
                }
                CardFileID::EventsData => {
                    debug!("DriverCard::parse - Application Identification: {:?}", application_identification);
                    if let Some(app_iden) = &application_identification {
                        let params = CardEventDataParams::new(6, app_iden.no_events_per_type);
                        card_event_data = Some(CardEventData::read(&mut reader, &params)?);
                    }
                }
                CardFileID::FaultsData
                | CardFileID::DriverActivityData
                | CardFileID::VehiclesUsed
                | CardFileID::Places
                | CardFileID::CurrentUsage
                | CardFileID::ControlActivityData
                | CardFileID::Identification => {
                    debug!("Not Implemented")
                }
                CardFileID::DrivingLicenseInfo => {
                    card_driving_license_info = Some(CardDrivingLicenceInformation::read(&mut reader)?);
                }
                CardFileID::SpecificConditions | CardFileID::CardCertificate | CardFileID::CACertificate => {
                    debug!("Not Implemented")
                }
                CardFileID::IC | CardFileID::ICC => trace!("Already parsed: {:?}", card_item.0),
                _ => trace!("Not Parsed: {:?}", card_item.0),
            }
        }

        // ApplicationIdentification is always there
        if application_identification.is_none() {
            return Err(Error::MissingCardFile(CardFileID::ApplicationIdentification.to_string()));
        }

        Ok(Box::new(Self {
            card_chip_identification,
            card_icc_identification,
            application_identification: application_identification.unwrap(),
            card_download,
            card_event_data,
            card_driving_license_info,
        }))
    }
}
