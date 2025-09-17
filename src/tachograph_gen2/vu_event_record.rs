use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::{FullCardNumberAndGeneration, ManufacturerSpecificEventFaultData},
    tacho::{EventFaultRecordPurpose, EventFaultType, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct VuEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_driver_slot_begin: FullCardNumberAndGeneration,
    pub card_number_codriver_slot_begin: FullCardNumberAndGeneration,
    pub card_number_driver_slot_end: FullCardNumberAndGeneration,
    pub card_number_codriver_slot_end: FullCardNumberAndGeneration,
    pub similar_events_number: u8,
    pub manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultData,
}

impl Readable<VuEventRecord> for VuEventRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuEventRecord> {
        let event_type: EventFaultType = reader.read_u8()?.into();
        let event_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let card_number_driver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_number_codriver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_number_driver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let card_number_codriver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let similar_events_number = reader.read_u8()?;
        let manufacturer_specific_event_fault_data = ManufacturerSpecificEventFaultData::read(reader)?;

        Ok(Self {
            event_type,
            event_record_purpose,
            event_begin_time,
            event_end_time,
            card_number_driver_slot_begin,
            card_number_codriver_slot_begin,
            card_number_driver_slot_end,
            card_number_codriver_slot_end,
            similar_events_number,
            manufacturer_specific_event_fault_data,
        })
    }
}
