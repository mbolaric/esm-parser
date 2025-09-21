use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{Address, Name, TimeReal},
};

/// Information, stored in a vehicle unit, related to one company lock
/// (Annex 1B requirement 104 and Annex 1C requirement 128).
#[derive(Debug, Serialize)]
pub struct VuCompanyLocksRecord {
    #[serde(rename = "lockInTime")]
    pub lock_in_time: TimeReal,
    #[serde(rename = "lockOutTime")]
    pub lock_out_time: TimeReal,
    #[serde(rename = "companyName")]
    pub company_name: Name,
    #[serde(rename = "companyAddress")]
    pub company_address: Address,
    #[serde(rename = "companyCardNumberAndGeneration")]
    pub company_card_number_and_generation: FullCardNumberAndGeneration,
}

impl Readable<VuCompanyLocksRecord> for VuCompanyLocksRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuCompanyLocksRecord> {
        let lock_in_time = TimeReal::read(reader)?;
        let lock_out_time = TimeReal::read(reader)?;
        let company_name = Name::read(reader)?;
        let company_address = Address::read(reader)?;
        let company_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        Ok(Self { lock_in_time, lock_out_time, company_name, company_address, company_card_number_and_generation })
    }
}
