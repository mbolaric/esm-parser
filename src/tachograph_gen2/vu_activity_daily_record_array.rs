use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    gen2::DataInfoReadable,
    tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, RecordType},
    tachograph_gen2::data_info::DataConfig,
};

/// Information, stored in a VU, related to changes of activity and/or
/// changes of driving status and/or changes of card status for a given
/// calendar day (Annex 1C requirement 105, 106, 107) and to slots status at 00:00 that day.
#[derive(Debug, Serialize)]
pub struct VuActivityDailyRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<ActivityChangeInfo>,
}

impl DataInfoReadable<VuActivityDailyRecordArray> for VuActivityDailyRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuActivityDailyRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<ActivityChangeInfo> = Vec::with_capacity(no_of_records as usize);
        let params = ActivityChangeInfoParams::new(ActivityCard::Vu);
        for _ in 0..no_of_records {
            let record = ActivityChangeInfo::read(reader, &params)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
