use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug)]
pub struct CompanyActivityDataParams {
    pub no_of_company_activity_records: u32,
}

impl CompanyActivityDataParams {
    pub fn new(no_of_company_activity_records: u32) -> Self {
        Self { no_of_company_activity_records }
    }
}

#[derive(Debug)]
pub struct CompanyActivityData<T> {
    pub company_pointer_newest_record: u16,
    pub company_activity_records: Vec<T>,
}

impl<T: Readable<T>> ReadableWithParams<CompanyActivityData<T>> for CompanyActivityData<T> {
    type P = CompanyActivityDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CompanyActivityData<T>> {
        let company_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut company_activity_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_company_activity_records {
            let record = T::read(reader)?;
            company_activity_records.push(record);
        }
        Ok(Self { company_pointer_newest_record, company_activity_records })
    }
}
