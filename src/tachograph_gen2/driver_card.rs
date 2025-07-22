use std::collections::HashMap;

use log::{debug, trace};

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{
        CardResponseParameterData, Certificate, CertificateParams, DriverCardApplicationIdentification, GnssAccumulatedDriving,
        GnssAccumulatedDrivingParams,
    },
    tacho::{Card, CardChipIdentification, CardDataFile, CardFileID, CardGeneration, CardIccIdentification, TimeReal},
};

#[derive(Debug)]
pub struct DriverCard {
    pub card_generation: CardGeneration,
    pub card_chip_identification: CardChipIdentification,
    pub card_icc_identification: CardIccIdentification,
    pub application_identification: DriverCardApplicationIdentification,
    pub card_download: Option<TimeReal>,
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
            card_certificate: None,
            gnss_places: None,
            ca_certificate: None,
            card_sign_certificate: None,
            link_certificate: None,
            card_notes,
        }
    }

    pub fn parse(card_data_files: &HashMap<CardFileID, CardDataFile>, card_notes: &str) -> Result<Box<DriverCard>> {
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
                CardFileID::EventsData
                | CardFileID::FaultsData
                | CardFileID::DriverActivityData
                | CardFileID::VehiclesUsed
                | CardFileID::Places
                | CardFileID::CurrentUsage
                | CardFileID::ControlActivityData
                | CardFileID::Identification
                | CardFileID::DrivingLicenseInfo
                | CardFileID::SpecificConditions
                | CardFileID::VehicleUnitsUsed => {
                    trace!("DriverCard::parse - Not Implemented: {:?}", card_item.0)
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
