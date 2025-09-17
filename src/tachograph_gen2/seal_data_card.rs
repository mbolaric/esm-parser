use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::SealRecord};

#[derive(Debug, Serialize)]
pub struct SealDataCard {
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
        Ok(Self { seal_records: records })
    }
}
