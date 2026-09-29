use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::SealRecord;
use crate::{Readable, Result, read_records};

/// This data type stores information about the seals that are attached to the
/// different components of a vehicle and is intended for storage in a Vehicle Unit.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SealDataVu"))]
pub struct SealDataVu {
    #[serde(rename = "sealRecords")]
    pub seal_records: Vec<SealRecord>,
}

impl Readable for SealDataVu {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealDataVu> {
        let seal_records = read_records(reader, 5, |reader| SealRecord::read(reader))?;
        Ok(Self { seal_records })
    }
}
