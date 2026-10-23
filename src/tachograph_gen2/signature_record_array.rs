use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::DataInfoReadable;
use crate::tacho::RecordType;
use crate::tachograph_gen2::data_info::DataConfig;
use crate::{Error, Result};

/// A set of signatures plus metadata used in the download protocol.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SignatureRecordArray"))]
pub struct SignatureRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<Vec<u8>>,
}

impl DataInfoReadable for SignatureRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<SignatureRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;
        config.validate_layout(RecordType::Signature, record_size)?;
        if !(64..=132).contains(&record_size) {
            return Err(Error::InvalidDataParse(format!("Invalid Gen2 signature size: {record_size}")));
        }

        let records = config.read_records(reader, |reader| Ok(reader.read_into_vec(record_size as u32)?))?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
