use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    CodePage, Result, bytes_to_string, gen2::DataInfoReadable, tacho::RecordType, tachograph_gen2::data_info::DataConfig,
};

/// The Vehicle Idenification Number plus metadata as used in the download protocol.
#[derive(Debug, Serialize)]
pub struct VehicleIdentificationNumberRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<String>,
}

impl DataInfoReadable<VehicleIdentificationNumberRecordArray> for VehicleIdentificationNumberRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleIdentificationNumberRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<String> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = bytes_to_string(&reader.read_into_vec(record_size as u32)?, &CodePage::IsoIec8859_1);
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
