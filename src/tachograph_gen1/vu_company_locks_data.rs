use serde::Serialize;

use crate::{
    Readable,
    tacho::{Address, FullCardNumber, Name, TimeReal},
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
    #[serde(rename = "companyCardNumber")]
    pub company_card_number: FullCardNumber,
}

impl Readable<VuCompanyLocksRecord> for VuCompanyLocksRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuCompanyLocksRecord> {
        let lock_in_time = TimeReal::read(reader)?;
        let lock_out_time = TimeReal::read(reader)?;
        let company_name = Name::read(reader)?;
        let company_address = Address::read(reader)?;
        let company_card_number = FullCardNumber::read(reader)?;

        Ok(Self { lock_in_time, lock_out_time, company_name, company_address, company_card_number })
    }
}

#[derive(Debug, Serialize)]
pub struct VuCompanyLocksData {
    pub no_of_locks: u8,
    pub company_locks: Vec<VuCompanyLocksRecord>,
}

impl Readable<VuCompanyLocksData> for VuCompanyLocksData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuCompanyLocksData> {
        let no_of_locks = reader.read_u8()?;
        let mut company_locks: Vec<VuCompanyLocksRecord> = Vec::new();
        for _ in 0..no_of_locks {
            company_locks.push(VuCompanyLocksRecord::read(reader)?);
        }

        Ok(Self { no_of_locks, company_locks })
    }
}
