use serde::Serialize;

use crate::{
    Readable,
    tacho::{EventFaultRecordPurpose, EventFaultType, FullCardNumber, TimeReal},
};

/// Information, stored in a vehicle unit, related to over speeding events
/// (Annex 1B requirement 094 and Annex 1C requirement 117).
#[derive(Debug, Serialize)]
pub struct VuOverSpeedingEventRecord {
    #[serde(rename = "eventType")]
    pub event_type: EventFaultType,
    #[serde(rename = "eventRecordPurpose")]
    pub event_record_purpose: EventFaultRecordPurpose,
    #[serde(rename = "eventBeginTime")]
    pub event_begin_time: TimeReal,
    #[serde(rename = "eventEndTime")]
    pub event_end_time: TimeReal,
    #[serde(rename = "maxSpeedValue")]
    pub max_speed_value: u8,
    #[serde(rename = "averageSpeedValue")]
    pub average_speed_value: u8,
    #[serde(rename = "cardNumberDriverSlotBegin")]
    pub card_number_driver_slot_begin: FullCardNumber,
    #[serde(rename = "similarEventsNumber")]
    pub similar_events_number: u8,
}

impl Readable<VuOverSpeedingEventRecord> for VuOverSpeedingEventRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingEventRecord> {
        let event_type: EventFaultType = reader.read_u8()?.into();
        let event_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let max_speed_value = reader.read_u8()?;
        let average_speed_value = reader.read_u8()?;
        let card_number_driver_slot_begin = FullCardNumber::read(reader)?;
        let similar_events_number = reader.read_u8()?;

        Ok(Self {
            event_type,
            event_record_purpose,
            event_begin_time,
            event_end_time,
            max_speed_value,
            average_speed_value,
            card_number_driver_slot_begin,
            similar_events_number,
        })
    }
}

/// Information, stored in a vehicle unit, related to over speeding events
/// (Annex 1B requirement 094).
#[derive(Debug, Serialize)]
pub struct VuOverSpeedingEventData {
    #[serde(rename = "noOfVuOverSpeedingEvents")]
    pub no_of_vu_over_speeding_events: u8,
    #[serde(rename = "vuOverSpeedingEventRecords")]
    pub vu_over_speeding_event_records: Vec<VuOverSpeedingEventRecord>,
}

impl Readable<VuOverSpeedingEventData> for VuOverSpeedingEventData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingEventData> {
        let no_of_vu_over_speeding_events = reader.read_u8()?;
        let mut vu_over_speeding_event_records: Vec<VuOverSpeedingEventRecord> = Vec::new();
        for _ in 0..no_of_vu_over_speeding_events {
            let record = VuOverSpeedingEventRecord::read(reader)?;
            vu_over_speeding_event_records.push(record);
        }
        Ok(Self { no_of_vu_over_speeding_events, vu_over_speeding_event_records })
    }
}
