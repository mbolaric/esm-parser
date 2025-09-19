use serde::Serialize;

use crate::{
    Readable,
    tacho::{CompanyActivityType, FullCardNumber, TimeReal, VehicleRegistrationIdentification},
};

/// Information, stored in a company card, related to activities performed
/// with the card (Annex 1C requirement 373 and 379).
#[derive(Debug, Serialize)]
pub struct CompanyActivityRecord {
    #[serde(rename = "companyActivityType")]
    pub company_activity_type: CompanyActivityType,
    #[serde(rename = "companyActivityTime")]
    pub company_activity_time: TimeReal,
    #[serde(rename = "cardNumberInformation")]
    pub card_number_information: FullCardNumber,
    #[serde(rename = "vehicleRegistrationInformation")]
    pub vehicle_registration_information: VehicleRegistrationIdentification,
    #[serde(rename = "downloadPeriodBegin")]
    pub download_period_begin: TimeReal,
    #[serde(rename = "downloadPeriodEnd")]
    pub download_period_end: TimeReal,
}

impl Readable<CompanyActivityRecord> for CompanyActivityRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyActivityRecord> {
        let company_activity_type: CompanyActivityType = reader.read_u8()?.into();
        let company_activity_time = TimeReal::read(reader)?;
        let card_number_information = FullCardNumber::read(reader)?;
        let vehicle_registration_information = VehicleRegistrationIdentification::read(reader)?;
        let download_period_begin = TimeReal::read(reader)?;
        let download_period_end = TimeReal::read(reader)?;

        Ok(Self {
            company_activity_type,
            company_activity_time,
            card_number_information,
            vehicle_registration_information,
            download_period_begin,
            download_period_end,
        })
    }
}
