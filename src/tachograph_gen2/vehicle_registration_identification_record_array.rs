use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::DataInfoReadable;
use crate::tacho::{RecordType, VehicleRegistrationIdentification};
use crate::tachograph_gen2::data_info::DataConfig;
use crate::{Readable, Result};

/// The Vehicle Registration Identification plus metadata as used in the download protocol.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VehicleRegistrationIdentificationRecordArray"))]
pub struct VehicleRegistrationIdentificationRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<VehicleRegistrationIdentification>,
}

impl DataInfoReadable<VehicleRegistrationIdentificationRecordArray> for VehicleRegistrationIdentificationRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleRegistrationIdentificationRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<VehicleRegistrationIdentification> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = VehicleRegistrationIdentification::read(reader)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
