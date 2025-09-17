use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    tacho::{EventFaultType, TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug, Serialize)]
pub struct CardFaultRecord {
    pub event_fault_type: EventFaultType,
    pub begin_time: TimeReal,
    pub end_time: TimeReal,
    pub fault_vehicle_registration: VehicleRegistrationIdentification,
}

impl Readable<CardFaultRecord> for CardFaultRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardFaultRecord> {
        let event_fault_type = reader.read_u8()?.into();
        let begin_time = TimeReal::read(reader)?;
        let end_time = TimeReal::read(reader)?;
        let fault_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;

        Ok(Self { event_fault_type, begin_time, end_time, fault_vehicle_registration })
    }
}

#[derive(Debug)]
pub struct CardFaultDataParams {
    pub no_faults_per_type: u8,
}

impl CardFaultDataParams {
    pub fn new(no_faults_per_type: u8) -> Self {
        Self { no_faults_per_type }
    }
}

#[derive(Debug)]
pub struct CardFaultData {
    pub no_faults_per_type: u8,
    pub fault_records: Vec<Vec<CardFaultRecord>>,
}

impl ReadableWithParams<CardFaultData> for CardFaultData {
    type P = CardFaultDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardFaultData> {
        let no_faults_per_type = params.no_faults_per_type;

        let mut fault_records: Vec<Vec<CardFaultRecord>> = Vec::new();
        for _ in 0..2 {
            let mut records: Vec<CardFaultRecord> = Vec::new();
            for _ in 0..no_faults_per_type {
                let record = CardFaultRecord::read(reader)?;
                if record.event_fault_type != EventFaultType::NoFurtherDetails
                    || record.begin_time.data != 0
                    || record.end_time.data != 0
                {
                    records.push(record);
                }
            }
            if !records.is_empty() {
                fault_records.push(records);
            }
        }
        Ok(Self { no_faults_per_type, fault_records })
    }
}
