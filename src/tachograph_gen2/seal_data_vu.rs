use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::SealRecord;
use crate::{Readable, Result};

/// This data type stores information about the seals that are attached to the
/// different components of a vehicle and is intended for storage in a Vehicle Unit.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SealDataVu"))]
pub struct SealDataVu {
    #[serde(rename = "sealRecords")]
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
