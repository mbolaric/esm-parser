use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

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

/// Information, stored in a driver or a workshop card, related to an event
/// associated to the card holder (Annex 1C requirements 261, 286, 318 and 341).
#[derive(Debug, Serialize)]
pub struct CardEventRecord {
    #[serde(rename = "eventType")]
    pub event_type: EventFaultType,
    #[serde(rename = "eventBeginTime")]
    pub event_begin_time: TimeReal,
    #[serde(rename = "eventEndTime")]
    pub event_end_time: TimeReal,
    #[serde(rename = "eventVehicleRegistration")]
    pub event_vehicle_registration: VehicleRegistrationIdentification,
}

impl Readable<CardEventRecord> for CardEventRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardEventRecord> {
        let event_type = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let event_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;

        Ok(Self { event_type, event_begin_time, event_end_time, event_vehicle_registration })
    }
}

/// This is a sequence, ordered by ascending value of EventFaultType,
/// of cardEventRecords (except security breach attempts related
/// records which are gathered in the last set of the sequence).
#[derive(Debug, Serialize)]
pub struct CardEventData {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u8,
    #[serde(rename = "cardEventRecords")]
    pub card_event_records: Vec<Vec<CardEventRecord>>,
}

impl ReadableWithParams<CardEventData> for CardEventData {
    type P = CardEventDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardEventData> {
        let no_of_records = params.no_of_records;
        let no_of_events_per_type = params.no_of_events_per_type;

        let mut card_event_records: Vec<Vec<CardEventRecord>> = Vec::new();
        for _ in 0..no_of_records {
            let mut records: Vec<CardEventRecord> = Vec::new();

            for _ in 0..no_of_events_per_type {
                let record = CardEventRecord::read(reader)?;
                if record.event_type != EventFaultType::NoFurtherDetails
                    || record.event_begin_time.get_data() != 0
                    || record.event_end_time.get_data() != 0
                {
                    records.push(record);
                }
            }
            if !records.is_empty() {
                card_event_records.push(records);
            }
        }

        Ok(Self { no_of_records, card_event_records })
    }
}
