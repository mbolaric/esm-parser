use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::SealRecord;
use crate::{Readable, Result, read_records};

/// This data type stores information about the seals that are attached to the
/// different components of a vehicle and is intended for storage on a card.
/// This data type is related to Annex 1C requirement 337.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SealDataCard"))]
pub struct SealDataCard {
    #[serde(rename = "noOfSealRecords")]
    pub no_of_seal_records: u8,
    #[serde(rename = "sealRecords")]
    pub seal_records: Vec<SealRecord>,
}

impl Readable for SealDataCard {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealDataCard> {
        let no_of_seal_records = reader.read_u8()?;
        let records = read_records(reader, no_of_seal_records as usize, |reader| SealRecord::read(reader))?;
        Ok(Self { no_of_seal_records, seal_records: records })
    }
}
