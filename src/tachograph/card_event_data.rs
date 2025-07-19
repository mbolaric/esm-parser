use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, ReadableWithParams, Result,
    tacho::{EventFaultType, TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct CardEventDataParams {
    pub no_of_records: u8,
    pub no_of_events_per_type: u8,
}

impl CardEventDataParams {
    pub fn new(no_of_records: u8, no_of_events_per_type: u8) -> Self {
        Self { no_of_records, no_of_events_per_type }
    }
}

#[derive(Debug)]
pub struct CardEventRecord {
    pub event_fault_type: EventFaultType,
    pub begin_time: TimeReal,
    pub end_time: TimeReal,
    pub vehicle_registration: VehicleRegistrationIdentification,
}

impl Readable<CardEventRecord> for CardEventRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardEventRecord> {
        let event_fault_type = reader.read_u8()?.into();
        let begin_time = TimeReal::read(reader)?;
        let end_time = TimeReal::read(reader)?;
        let vehicle_registration = VehicleRegistrationIdentification::read(reader)?;

        Ok(Self { event_fault_type, begin_time, end_time, vehicle_registration })
    }
}

#[derive(Debug)]
pub struct CardEventData {
    pub no_of_records: u8,
    pub event_records: Vec<Vec<CardEventRecord>>,
}

impl ReadableWithParams<CardEventData> for CardEventData {
    type P = CardEventDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardEventData> {
        let no_of_records = params.no_of_records;
        let no_of_events_per_type = params.no_of_events_per_type;

        let mut event_records: Vec<Vec<CardEventRecord>> = Vec::new();
        for _ in 0..no_of_records {
            let mut records: Vec<CardEventRecord> = Vec::new();

            for _ in 0..no_of_events_per_type {
                let record = CardEventRecord::read(reader)?;
                if record.event_fault_type != EventFaultType::NoFurtherDetails
                    || record.begin_time.get_data() != 0
                    || record.end_time.get_data() != 0
                {
                    records.push(record);
                }
            }
            if !records.is_empty() {
                event_records.push(records);
            }
        }

        Ok(Self { no_of_records, event_records })
    }
}
