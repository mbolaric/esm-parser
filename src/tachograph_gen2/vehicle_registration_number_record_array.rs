use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::DataInfoReadable,
    tacho::{RecordType, VehicleRegistrationNumber},
    tachograph_gen2::{
        data_info::DataConfig, vehicle_registration_identification_record_array::VehicleRegistrationIdentificationRecordArray,
    },
};

#[derive(Debug, Serialize)]
pub struct VehicleRegistrationNumberRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<VehicleRegistrationNumber>,
}

impl DataInfoReadable<VehicleRegistrationNumberRecordArray> for VehicleRegistrationNumberRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleRegistrationNumberRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<VehicleRegistrationNumber> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = VehicleRegistrationNumber::read(reader)?;
            records.push(record);
        }
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
