use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{EventFaultRecordPurpose, EventFaultType, TimeReal},
};

#[derive(Debug)]
pub struct VuOverSpeedingEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub max_speed_value: u8,
    pub average_speed_value: u8,
    pub card_number_driver_slot_begin: FullCardNumberAndGeneration,
    pub similar_events_number: u8,
}

impl Readable<VuOverSpeedingEventRecord> for VuOverSpeedingEventRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuOverSpeedingEventRecord> {
        let event_type: EventFaultType = reader.read_u8()?.into();
        let event_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let max_speed_value = reader.read_u8()?;
        let average_speed_value = reader.read_u8()?;
        let card_number_driver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
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
