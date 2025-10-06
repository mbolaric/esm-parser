use std::collections::HashMap;

use binary_data::BinSeek;
use log::{debug, trace};
use serde::Serialize;

use crate::gen2::{CardResponseParameterData, Certificate, CertificateParams, CompanyCardApplicationIdentificationV2};
use crate::tacho::{
    Card, CardChipIdentification, CardFileData, CardFileID, CardGeneration, CardIccIdentification, CardParser,
    CompanyActivityData, CompanyActivityDataParams, CompanyActivityRecord, CompanyCardApplicationIdentification, Identification,
    IdentificationParams,
};
use crate::{Readable, ReadableWithParams, Result};

/// Company card application generation 2
#[derive(Debug, Serialize)]
pub struct CompanyCard {
    #[serde(rename = "cardGeneration")]
    pub card_generation: CardGeneration,
    #[serde(rename = "cardChipIdentification")]
    pub card_chip_identification: CardChipIdentification,
    #[serde(rename = "cardIccIdentification")]
    pub card_icc_identification: CardIccIdentification,
    #[serde(rename = "applicationIdentification")]
    pub application_identification: CompanyCardApplicationIdentification,
    #[serde(rename = "applicationIdentificationV2")]
    pub application_identification_v2: Option<CompanyCardApplicationIdentificationV2>,
    pub identification: Option<Identification>,
    #[serde(rename = "companyActivityData")]
    pub company_activity_data: Option<CompanyActivityData<CompanyActivityRecord>>,
    #[serde(rename = "cardCertificate")]
    pub card_certificate: Option<Certificate>,
    #[serde(rename = "caCertificate")]
    pub ca_certificate: Option<Certificate>,
    #[serde(rename = "linkCertificate")]
    pub link_certificate: Option<Certificate>,
    #[serde(rename = "cardNotes")]
    pub card_notes: String,
    #[serde(rename = "dataFiles")]
    pub data_files: HashMap<CardFileID, CardFileData>,
}

impl CompanyCard {
    fn new(
        card_chip_identification: CardChipIdentification,
        card_icc_identification: CardIccIdentification,
        application_identification: CompanyCardApplicationIdentification,
        card_notes: String,
        data_files: HashMap<CardFileID, CardFileData>,
    ) -> Self {
        Self {
            card_generation: CardGeneration::Gen1,
            card_chip_identification,
            card_icc_identification,
            application_identification,
            application_identification_v2: None,
            identification: None,
            company_activity_data: None,
            card_certificate: None,
            ca_certificate: None,
            link_certificate: None,
            card_notes,
            data_files,
        }
    }
}

impl CardParser<CompanyCard> for CompanyCard {
    fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<CompanyCard>> {
        let card_chip_identification = <dyn Card<CardResponseParameterData>>::parse_ic(card_data_files)?;
        let card_icc_identification = <dyn Card<CardResponseParameterData>>::parse_icc(card_data_files)?;
        let application_identification = <dyn Card<CardResponseParameterData>>::parse_card_application_identification::<
            CompanyCardApplicationIdentification,
        >(card_data_files)?;

        let mut company_card = CompanyCard::new(
            card_chip_identification,
            card_icc_identification,
            application_identification.clone(),
            card_notes.to_owned(),
            (*card_data_files).clone(),
        );

        for card_item in card_data_files.iter() {
            debug!("CompanyCard::parse - ID: {:?}", card_item.0,);
            let card_file = card_item.1;
            let mut reader = card_file.data_into_reader()?;
            debug!(
                "CompanyCard::parse - ID: {:?}, Data Length: {:?}, Has Signature: {}",
                card_item.0,
                reader.len()?,
                card_file.signature.is_some()
            );
            match card_item.0 {
                CardFileID::ApplicationIdentificationV2 => {
                    company_card.application_identification_v2 = Some(CompanyCardApplicationIdentificationV2::read(&mut reader)?);
                }
                CardFileID::Identification => {
                    let params = IdentificationParams::new(application_identification.type_of_tachograph_card_id.clone());
                    company_card.identification = Some(Identification::read(&mut reader, &params)?);
                }
                CardFileID::CompanyActivityData => {
                    let params = CompanyActivityDataParams::new(application_identification.no_of_company_activity_records);
                    company_card.company_activity_data = Some(CompanyActivityData::read(&mut reader, &params)?);
                }
                CardFileID::CardCertificate => {
                    let params = CertificateParams::new(None);
                    company_card.ca_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::CACertificate => {
                    let params = CertificateParams::new(None);
                    company_card.card_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::LinkCertificate => {
                    let params = CertificateParams::new(None);
                    company_card.link_certificate = Some(Certificate::read(&mut reader, &params)?);
                }
                CardFileID::IC | CardFileID::ICC | CardFileID::ApplicationIdentification => {
                    trace!("CompanyCard::parse - Already parsed: {:?}", card_item.0)
                }
                _ => trace!("CompanyCard::parse - Not Parsed: {:?}", card_item.0),
            }
        }

        Ok(Box::new(company_card))
    }
}
