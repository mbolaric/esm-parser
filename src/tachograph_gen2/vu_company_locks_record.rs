use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{Address, Name, TimeReal},
};

#[derive(Debug)]
pub struct VuCompanyLocksRecord {
    pub lock_in_time: TimeReal,
    pub lock_out_time: TimeReal,
    pub company_name: Name,
    pub company_address: Address,
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
