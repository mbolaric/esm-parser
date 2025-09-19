use serde::Serialize;

use crate::{
    Readable,
    tacho::{FullCardNumber, Name, TimeReal},
};

/// Information, stored in a vehicle unit, related to its last download (Annex
/// 1B requirement 105 and Annex 1C requirement 129).
#[derive(Debug, Serialize)]
pub struct VuDownloadActivityData {
    #[serde(rename = "downloadingTime")]
    pub downloading_time: TimeReal,
    #[serde(rename = "fullCardNumber")]
    pub full_card_number: FullCardNumber,
    #[serde(rename = "companyOrWorkshopName")]
    pub company_or_workshop_name: Name,
}

impl Readable<VuDownloadActivityData> for VuDownloadActivityData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDownloadActivityData> {
        let downloading_time = TimeReal::read(reader)?;
        let full_card_number = FullCardNumber::read(reader)?;
        let company_or_workshop_name = Name::read(reader)?;

        Ok(Self { downloading_time, full_card_number, company_or_workshop_name })
    }
}
