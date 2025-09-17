use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{Name, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct VuDownloadActivityData {
    pub downloading_time: TimeReal,
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub company_or_workshop_name: Name,
}

impl Readable<VuDownloadActivityData> for VuDownloadActivityData {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuDownloadActivityData> {
        let downloading_time = TimeReal::read(reader)?;
        let full_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let company_or_workshop_name = Name::read(reader)?;
        Ok(Self { downloading_time, full_card_number_and_generation, company_or_workshop_name })
    }
}
