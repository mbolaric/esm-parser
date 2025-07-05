use crate::{
    Readable,
    tacho::{EventFaultRecordPurpose, EventFaultType, FullCardNumber, TimeReal},
};

#[derive(Debug)]
pub struct VuEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_driver_slot_begin: FullCardNumber,
    pub card_number_codriver_slot_begin: FullCardNumber,
    pub card_number_driver_slot_end: FullCardNumber,
    pub card_number_codriver_slot_end: FullCardNumber,
    pub similar_events_number: u8,
}

impl Readable<VuEventRecord> for VuEventRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuEventRecord> {
        let event_type: EventFaultType = reader.read_u8()?.into();
        let event_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let event_begin_time = TimeReal::read(reader)?;
        let event_end_time = TimeReal::read(reader)?;
        let card_number_driver_slot_begin = FullCardNumber::read(reader)?;
        let card_number_codriver_slot_begin = FullCardNumber::read(reader)?;
        let card_number_driver_slot_end = FullCardNumber::read(reader)?;
        let card_number_codriver_slot_end = FullCardNumber::read(reader)?;
        let similar_events_number = reader.read_u8()?;

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
        })
    }
}

#[derive(Debug)]
pub struct VuEventData {
    pub no_of_vu_events: u8,
    pub vu_event_records: Vec<VuEventRecord>,
}

impl Readable<VuEventData> for VuEventData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuEventData> {
        let no_of_vu_events = reader.read_u8()?;
        let mut vu_event_records: Vec<VuEventRecord> = Vec::new();
        for _ in 0..no_of_vu_events {
            let record = VuEventRecord::read(reader)?;
            vu_event_records.push(record);
        }

        Ok(Self { no_of_vu_events, vu_event_records })
    }
}
