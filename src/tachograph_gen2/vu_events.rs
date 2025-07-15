use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{DataInfo, DataInfoGenericRecords, VuFaultRecord};
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUEvents {
    pub vu_fault_records: DataInfoGenericRecords<VuFaultRecord>,
}

impl VUEvents {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUEvents> {
        debug!("VUEvents::from_data - Trep ID: {:?}", trep_id);
        let vu_fault_records: DataInfoGenericRecords<VuFaultRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let event = DataInfo::read(reader, trep_id.clone())?;
        let over_speeding_control_data = DataInfo::read(reader, trep_id.clone())?;
        let over_speeding_event = DataInfo::read(reader, trep_id.clone())?;
        let time_adjustment = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self { vu_fault_records })
    }
}
