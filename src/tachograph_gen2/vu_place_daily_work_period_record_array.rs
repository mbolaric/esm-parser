use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::{DataInfoReadable, FullCardNumberAndGeneration, PlaceRecord},
    tacho::{RecordType, VUTransferResponseParameterID},
    tachograph_gen2::data_info::DataConfig,
};

/// Information, stored in a vehicle unit, related to a place where a driver
/// begins or ends a daily work period (Annex 1B requirement 087 and
/// Annex 1C requirement 108 and 110).
#[derive(Debug, Serialize)]
pub struct VuPlaceDailyWorkPeriodRecord {
    #[serde(rename = "fullCardNumberAndGeneration")]
    pub full_card_number: FullCardNumberAndGeneration,
    #[serde(rename = "placeRecord")]
    pub place_record: PlaceRecord,
}

impl Readable<VuPlaceDailyWorkPeriodRecord> for VuPlaceDailyWorkPeriodRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuPlaceDailyWorkPeriodRecord> {
        let full_card_number = FullCardNumberAndGeneration::read(reader)?;
        let place_record = PlaceRecord::read(reader)?;
        Ok(Self { full_card_number, place_record })
    }
}

#[derive(Debug, Serialize)]
pub struct VuPlaceDailyWorkPeriodRecordArray {
    #[serde(rename = "isGen2V2")]
    pub is_gen2_v2: bool,
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<VuPlaceDailyWorkPeriodRecord>,
}

impl DataInfoReadable<VuPlaceDailyWorkPeriodRecordArray> for VuPlaceDailyWorkPeriodRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuPlaceDailyWorkPeriodRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<VuPlaceDailyWorkPeriodRecord> = Vec::with_capacity(no_of_records as usize);
        let is_gen2_v2: bool = config.trep_id == VUTransferResponseParameterID::Gen2v2Activities;
        for _ in 0..no_of_records {
            let record = VuPlaceDailyWorkPeriodRecord::read(reader)?;
            records.push(record);
            if is_gen2_v2 {
                // Instead of placeRecord, the generation 2 version 2 data structure makes
                // use of the following data element:
                // placeAuthRecord contains the information related to the place entered,
                // the recorded position, GNSS authentication status and position determination time.
                let _ = reader.read_u8()?;
            }
        }
        Ok(Self { is_gen2_v2, no_of_records, record_size, record_type, records })
    }
}
