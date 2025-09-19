use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::{DataInfoReadable, FullCardNumberAndGeneration, PlaceRecord},
    tacho::{RecordType, VUTransferResponseParameterID},
    tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug, Serialize)]
pub struct VuPlaceDailyWorkPeriodRecord {
    pub full_card_number: FullCardNumberAndGeneration,
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
pub struct VuPlaceDailyWorkPeriodRecords {
    pub is_gen2_v2: bool,
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: RecordType,
    pub records: Vec<VuPlaceDailyWorkPeriodRecord>,
}

impl DataInfoReadable<VuPlaceDailyWorkPeriodRecords> for VuPlaceDailyWorkPeriodRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuPlaceDailyWorkPeriodRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<VuPlaceDailyWorkPeriodRecord> = Vec::with_capacity(no_of_records as usize);
        let is_gen2_v2: bool = config.trep_id == VUTransferResponseParameterID::Gen2v2Activities;
        for _ in 0..no_of_records {
            let record = VuPlaceDailyWorkPeriodRecord::read(reader)?;
            records.push(record);
            if is_gen2_v2 {
                let _ = reader.read_u8()?;
            }
        }
        Ok(Self { is_gen2_v2, no_of_records, record_size, data_type_id, records })
    }
}
