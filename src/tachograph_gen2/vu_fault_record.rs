use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::{FullCardNumberAndGeneration, ManufacturerSpecificEventFaultData},
    tacho::{EventFaultRecordPurpose, EventFaultType, TimeReal},
};

#[derive(Debug)]
pub struct VuFaultRecord {
    pub fault_type: EventFaultType,
    pub fault_record_purpose: EventFaultRecordPurpose,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub card_number_driver_slot_begin: FullCardNumberAndGeneration,
    pub card_number_codriver_slot_begin: FullCardNumberAndGeneration,
    pub card_number_driver_slot_end: FullCardNumberAndGeneration,
    pub card_number_codriver_slot_end: FullCardNumberAndGeneration,
    pub manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultData,
}

impl Readable<VuFaultRecord> for VuFaultRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuFaultRecord> {
        let fault_type: EventFaultType = reader.read_u8()?.into();
        let fault_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let fault_begin_time = TimeReal::read(reader)?;
        let fault_end_time = TimeReal::read(reader)?;
        let card_number_driver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_number_codriver_slot_begin = FullCardNumberAndGeneration::read(reader)?;
        let card_number_driver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let card_number_codriver_slot_end = FullCardNumberAndGeneration::read(reader)?;
        let manufacturer_specific_event_fault_data = ManufacturerSpecificEventFaultData::read(reader)?;

        Ok(Self {
            fault_type,
            fault_record_purpose,
            fault_begin_time,
            fault_end_time,
            card_number_driver_slot_begin,
            card_number_codriver_slot_begin,
            card_number_driver_slot_end,
            card_number_codriver_slot_end,
            manufacturer_specific_event_fault_data,
        })
    }
}
