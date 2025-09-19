use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    gen2::DataInfoReadable,
    tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams, RecordType},
    tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug, Serialize)]
pub struct VuActivityDailyRecords {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: RecordType,
    pub records: Vec<ActivityChangeInfo>,
}

impl DataInfoReadable<VuActivityDailyRecords> for VuActivityDailyRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuActivityDailyRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<ActivityChangeInfo> = Vec::with_capacity(no_of_records as usize);
        let params = ActivityChangeInfoParams::new(ActivityCard::Vu);
        for _ in 0..no_of_records {
            let record = ActivityChangeInfo::read(reader, &params)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}
