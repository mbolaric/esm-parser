use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::DataInfoReadable;
use crate::tacho::RecordType;
use crate::tachograph_gen2::data_info::DataConfig;
use crate::{CodePage, Result, bytes_to_string, read_records};

/// The Vehicle Idenification Number plus metadata as used in the download protocol.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VehicleIdentificationNumberRecordArray"))]
pub struct VehicleIdentificationNumberRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<String>,
}

impl DataInfoReadable for VehicleIdentificationNumberRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleIdentificationNumberRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;

        let records = read_records(reader, no_of_records as usize, |reader| {
            Ok(bytes_to_string(&reader.read_into_vec(record_size as u32)?, &CodePage::IsoIec8859_1))
        })?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
