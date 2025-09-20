use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::SealRecord};

/// This data type stores information about the seals that are attached to the
/// different components of a vehicle and is intended for storage on a card.
/// This data type is related to Annex 1C requirement 337.
#[derive(Debug, Serialize)]
pub struct SealDataCard {
    #[serde(rename = "noOfSealRecords")]
    pub no_of_seal_records: u8,
    #[serde(rename = "sealRecords")]
    pub seal_records: Vec<SealRecord>,
}

impl Readable<SealDataCard> for SealDataCard {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealDataCard> {
        let no_of_seal_records = reader.read_u8()?;
        let mut records: Vec<SealRecord> = Vec::new();
        for _ in 0..no_of_seal_records {
            let record = SealRecord::read(reader)?;
            records.push(record);
        }
        Ok(Self { no_of_seal_records, seal_records: records })
    }
}
