use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::SealRecord};

#[derive(Debug, Serialize)]
pub struct SealDataVu {
    pub seal_records: Vec<SealRecord>,
}

impl Readable<SealDataVu> for SealDataVu {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealDataVu> {
        let mut seal_records: Vec<SealRecord> = Vec::with_capacity(5);
        for _ in 0..5 {
            let record = SealRecord::read(reader)?;
            seal_records.push(record);
        }
        Ok(Self { seal_records })
    }
}
