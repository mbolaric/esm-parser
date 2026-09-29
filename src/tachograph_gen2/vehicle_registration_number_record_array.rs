use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::DataInfoReadable;
use crate::tacho::{RecordType, VehicleRegistrationNumber};
use crate::tachograph_gen2::data_info::DataConfig;
use crate::tachograph_gen2::vehicle_registration_identification_record_array::VehicleRegistrationIdentificationRecordArray;
use crate::{Readable, Result, read_records};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VehicleRegistrationNumberRecordArray"))]
pub struct VehicleRegistrationNumberRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<VehicleRegistrationNumber>,
}

impl DataInfoReadable for VehicleRegistrationNumberRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleRegistrationNumberRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;

        let records = read_records(reader, no_of_records as usize, |reader| VehicleRegistrationNumber::read(reader))?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}

impl From<VehicleRegistrationIdentificationRecordArray> for VehicleRegistrationNumberRecordArray {
    fn from(value: VehicleRegistrationIdentificationRecordArray) -> Self {
        Self {
            record_type: value.record_type,
            no_of_records: value.no_of_records,
            record_size: value.record_size,
            records: value.records.into_iter().map(|item| item.vehicle_registration_number).collect(),
        }
    }
}
