use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::DataInfoReadable,
    tacho::{RecordType, VehicleRegistrationNumber},
    tachograph_gen2::{
        data_info::DataConfig, vehicle_registration_identification_records::VehicleRegistrationIdentificationRecords,
    },
};

#[derive(Debug, Serialize)]
pub struct VuVehicleRegistrationNumberRecords {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: RecordType,
    pub records: Vec<VehicleRegistrationNumber>,
}

impl DataInfoReadable<VuVehicleRegistrationNumberRecords> for VuVehicleRegistrationNumberRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuVehicleRegistrationNumberRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<VehicleRegistrationNumber> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = VehicleRegistrationNumber::read(reader)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}

impl From<VehicleRegistrationIdentificationRecords> for VuVehicleRegistrationNumberRecords {
    fn from(value: VehicleRegistrationIdentificationRecords) -> Self {
        Self {
            data_type_id: value.record_type,
            no_of_records: value.no_of_records,
            record_size: value.record_size,
            records: value.records.into_iter().map(|item| item.vehicle_registration_number).collect(),
        }
    }
}
