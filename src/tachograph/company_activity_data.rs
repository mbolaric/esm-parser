use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, read_records};

#[derive(Debug)]
pub struct CompanyActivityDataParams {
    pub no_of_company_activity_records: u32,
}

impl CompanyActivityDataParams {
    pub fn new(no_of_company_activity_records: u32) -> Self {
        Self { no_of_company_activity_records }
    }
}

/// Information, stored in a company card, related to activities performed
/// with the card (Annex 1C requirement 373 and 379).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CompanyActivityData<T> {
    #[serde(rename = "companyPointerNewestRecord")]
    pub company_pointer_newest_record: u16,
    #[serde(rename = "companyActivityRecords")]
    pub company_activity_records: Vec<T>,
}

impl<T: Readable> ReadableWithParams for CompanyActivityData<T> {
    type P = CompanyActivityDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CompanyActivityData<T>> {
        let company_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let company_activity_records =
            read_records(reader, params.no_of_company_activity_records as usize, |reader| T::read(reader))?;
        Ok(Self { company_pointer_newest_record, company_activity_records })
    }
}
