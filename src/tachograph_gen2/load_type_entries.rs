use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::LoadType;
use crate::tacho::TimeReal;
use crate::{Readable, ReadableWithParams, Result};

/// Information, stored in a driver or workshop card, related to load type entries
/// (Annex IC requirements 306j and 356j)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadTypeEntryRecord"))]
pub struct LoadTypeEntryRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "loadTypeEntered")]
    pub load_type_entered: LoadType,
}

impl Readable<LoadTypeEntryRecord> for LoadTypeEntryRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<LoadTypeEntryRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let load_type_entered: LoadType = reader.read_u8()?.into();
        Ok(Self { time_stamp, load_type_entered })
    }
}

#[derive(Debug)]
pub struct LoadTypeEntriesParams {
    pub no_of_load_type_entry_records: u16,
}

impl LoadTypeEntriesParams {
    pub fn new(no_of_load_type_entry_records: u16) -> Self {
        Self { no_of_load_type_entry_records }
    }
}

/// Information, stored in a driver or workshop card, related to load type entries
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadTypeEntries"))]
pub struct LoadTypeEntries {
    #[serde(rename = "loadTypeEntryPointerNewestRecord")]
    pub load_type_entry_pointer_newest_record: u16,
    #[serde(rename = "cardLoadTypeEntryRecords")]
    pub card_load_type_entry_records: Vec<LoadTypeEntryRecord>,
}

impl ReadableWithParams<LoadTypeEntries> for LoadTypeEntries {
    type P = LoadTypeEntriesParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<LoadTypeEntries> {
        let load_type_entry_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut records: Vec<LoadTypeEntryRecord> = Vec::new();
        for _ in 0..params.no_of_load_type_entry_records {
            let record = LoadTypeEntryRecord::read(reader)?;
            if record.time_stamp.has_data() {
                records.push(record);
            }
        }

        Ok(Self { load_type_entry_pointer_newest_record, card_load_type_entry_records: records })
    }
}
