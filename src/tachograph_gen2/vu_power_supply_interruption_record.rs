use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{EventFaultRecordPurpose, EventFaultType, TimeReal},
};

#[derive(Debug)]
pub struct VuPowerSupplyInterruptionRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_num_and_gen_driver_slot_begin: FullCardNumberAndGeneration,
    pub card_num_and_gen_driver_slot_end: FullCardNumberAndGeneration,
    pub card_num_and_gen_codriver_slot_begin: FullCardNumberAndGeneration,
    pub card_num_and_gen_codriver_slot_end: FullCardNumberAndGeneration,
    pub similar_events: u8,
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
        let similar_events = reader.read_u8()?;

        Ok(Self {
            event_type,
            event_record_purpose,
            event_begin_time,
            event_end_time,
            card_num_and_gen_driver_slot_begin,
            card_num_and_gen_driver_slot_end,
            card_num_and_gen_codriver_slot_begin,
            card_num_and_gen_codriver_slot_end,
            similar_events,
        })
    }
}
