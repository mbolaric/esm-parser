use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Result, gen2::DataInfoReadable, tacho::RecordType, tachograph_gen2::data_info::DataConfig};

/// A set of signatures plus metadata used in the download protocol.
#[derive(Debug, Serialize)]
pub struct SignatureRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<Vec<u8>>,
}

impl DataInfoReadable<SignatureRecordArray> for SignatureRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<SignatureRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<Vec<u8>> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = reader.read_into_vec(record_size as u32)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
