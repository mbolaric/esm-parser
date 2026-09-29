use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::DataInfoReadable;
use crate::tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, RecordType};
use crate::tachograph_gen2::data_info::DataConfig;
use crate::{ReadableWithParams, Result, read_records};

/// Information, stored in a VU, related to changes of activity and/or
/// changes of driving status and/or changes of card status for a given
/// calendar day (Annex 1C requirement 105, 106, 107) and to slots status at 00:00 that day.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuActivityDailyRecordArray"))]
pub struct VuActivityDailyRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<ActivityChangeInfo>,
}

impl DataInfoReadable for VuActivityDailyRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuActivityDailyRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;

        let params = ActivityChangeInfoParams::new(ActivityCard::Vu);
        let records = read_records(reader, no_of_records as usize, |reader| ActivityChangeInfo::read(reader, &params))?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
