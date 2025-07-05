use crate::{
    Readable,
    tacho::{EventFaultRecordPurpose, EventFaultType, FullCardNumber, TimeReal},
};

#[derive(Debug)]
pub struct VuFaultRecord {
    pub fault_type: EventFaultType,
    pub fault_record_purpose: EventFaultRecordPurpose,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub card_number_driver_slot_begin: FullCardNumber,
    pub card_number_codriver_slot_begin: FullCardNumber,
    pub card_number_driver_slot_end: FullCardNumber,
    pub card_number_codriver_slot_end: FullCardNumber,
}

impl Readable<VuFaultRecord> for VuFaultRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuFaultRecord> {
        let fault_type: EventFaultType = reader.read_u8()?.into();
        let fault_record_purpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let fault_begin_time = TimeReal::read(reader)?;
        let fault_end_time = TimeReal::read(reader)?;
        let card_number_driver_slot_begin = FullCardNumber::read(reader)?;
        let card_number_codriver_slot_begin = FullCardNumber::read(reader)?;
        let card_number_driver_slot_end = FullCardNumber::read(reader)?;
        let card_number_codriver_slot_end = FullCardNumber::read(reader)?;

        Ok(Self {
            fault_type,
            fault_record_purpose,
            fault_begin_time,
            fault_end_time,
            card_number_driver_slot_begin,
            card_number_codriver_slot_begin,
            card_number_driver_slot_end,
            card_number_codriver_slot_end,
        })
    }
}

#[derive(Debug)]
pub struct VuFaultData {
    pub no_of_vu_faults: u8,
    pub vu_fault_records: Vec<VuFaultRecord>,
}

impl Readable<VuFaultData> for VuFaultData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuFaultData> {
        let no_of_vu_faults = reader.read_u8()?;
        let mut vu_fault_records: Vec<VuFaultRecord> = Vec::new();
        for _ in 0..no_of_vu_faults {
            let record = VuFaultRecord::read(reader)?;
            vu_fault_records.push(record);
        }

        Ok(Self { no_of_vu_faults, vu_fault_records })
    }
}
