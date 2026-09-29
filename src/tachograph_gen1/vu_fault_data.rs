use serde::Serialize;

use crate::tacho::{EventFaultRecordPurpose, EventFaultType, FullCardNumber, TimeReal};
use crate::{Readable, read_records};

/// Information, stored in a vehicle unit, related to a fault (Annex 1B
/// requirement 096 and Annex 1C requirement 118).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuFaultRecord"))]
pub struct VuFaultRecord {
    #[serde(rename = "faultType")]
    pub fault_type: EventFaultType,
    #[serde(rename = "faultRecordPurpose")]
    pub fault_record_purpose: EventFaultRecordPurpose,
    #[serde(rename = "faultBeginTime")]
    pub fault_begin_time: TimeReal,
    #[serde(rename = "faultEndTime")]
    pub fault_end_time: TimeReal,
    #[serde(rename = "cardNumberDriverSlotBegin")]
    pub card_number_driver_slot_begin: FullCardNumber,
    #[serde(rename = "cardNumberCodriverSlotBegin")]
    pub card_number_codriver_slot_begin: FullCardNumber,
    #[serde(rename = "cardNumberDriverSlotEnd")]
    pub card_number_driver_slot_end: FullCardNumber,
    #[serde(rename = "cardNumberCodriverSlotEnd")]
    pub card_number_codriver_slot_end: FullCardNumber,
}

impl Readable for VuFaultRecord {
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

/// Information, stored in a vehicle unit, related to faults (Annex 1B requirement 096).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuFaultData"))]
pub struct VuFaultData {
    #[serde(rename = "noOfVuFaults")]
    pub no_of_vu_faults: u8,
    #[serde(rename = "vuFaultRecords")]
    pub vu_fault_records: Vec<VuFaultRecord>,
}

impl Readable for VuFaultData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuFaultData> {
        let no_of_vu_faults = reader.read_u8()?;
        let vu_fault_records = read_records(reader, no_of_vu_faults as usize, |reader| VuFaultRecord::read(reader))?;

        Ok(Self { no_of_vu_faults, vu_fault_records })
    }
}
