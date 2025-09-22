use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{EventFaultRecordPurpose, EventFaultType, TimeReal},
};

/// Information, stored in a vehicle unit, related to Power Supply Interruption
/// events (Annex 1C requirement 117).
#[derive(Debug, Serialize)]
pub struct VuPowerSupplyInterruptionRecord {
    #[serde(rename = "eventType")]
    pub event_type: EventFaultType,
    #[serde(rename = "eventRecordPurpose")]
    pub event_record_purpose: EventFaultRecordPurpose,
    #[serde(rename = "eventBeginTime")]
    pub event_begin_time: TimeReal,
    #[serde(rename = "eventEndTime")]
    pub event_end_time: TimeReal,
    #[serde(rename = "cardNumberAndGenDriverSlotBegin")]
    pub card_num_and_gen_driver_slot_begin: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenDriverSlotEnd")]
    pub card_num_and_gen_driver_slot_end: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenCodriverSlotBegin")]
    pub card_num_and_gen_codriver_slot_begin: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenCodriverSlotEnd")]
    pub card_num_and_gen_codriver_slot_end: FullCardNumberAndGeneration,
    #[serde(rename = "similarEventsNumber")]
    pub similar_events_number: u8,
}

impl Readable<VuPowerSupplyInterruptionRecord> for VuPowerSupplyInterruptionRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuPowerSupplyInterruptionRecord> {
        let event_type: EventFaultType = reader.read_u8()?.into();
        let event_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let card_num_and_gen_driver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_num_and_gen_driver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let card_num_and_gen_codriver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_num_and_gen_codriver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let similar_events_number = reader.read_u8()?;

        Ok(Self {
            event_type,
            event_record_purpose,
            event_begin_time,
            event_end_time,
            card_num_and_gen_driver_slot_begin,
            card_num_and_gen_driver_slot_end,
            card_num_and_gen_codriver_slot_begin,
            card_num_and_gen_codriver_slot_end,
            similar_events_number,
        })
    }
}
