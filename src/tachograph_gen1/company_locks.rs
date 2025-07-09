use crate::{
    Readable,
    tacho::{Address, FullCardNumber, Name, TimeReal},
};

#[derive(Debug)]
pub struct CompanyLocksRecord {
    pub lock_in_time: TimeReal,
    pub lock_out_time: TimeReal,
    pub company_or_workshop_name: Name,
    pub company_address: Address,
    pub full_card_number: FullCardNumber,
}

impl Readable<CompanyLocksRecord> for CompanyLocksRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyLocksRecord> {
        let lock_in_time = TimeReal::read(reader)?;
        let lock_out_time = TimeReal::read(reader)?;
        let company_or_workshop_name = Name::read(reader)?;
        let company_address = Address::read(reader)?;
        let full_card_number = FullCardNumber::read(reader)?;

        Ok(Self { lock_in_time, lock_out_time, company_or_workshop_name, company_address, full_card_number })
    }
}

#[derive(Debug)]
pub struct CompanyLocks {
    pub no_of_locks: u8,
    pub company_locks: Vec<CompanyLocksRecord>,
}

impl Readable<CompanyLocks> for CompanyLocks {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyLocks> {
        let no_of_locks = reader.read_u8()?;
        let mut company_locks: Vec<CompanyLocksRecord> = Vec::new();
        for _ in 0..no_of_locks {
            company_locks.push(CompanyLocksRecord::read(reader)?);
        }

        Ok(Self { no_of_locks, company_locks })
    }
}
