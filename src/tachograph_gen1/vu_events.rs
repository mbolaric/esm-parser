use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::{VuEventData, VuFaultData, VuOverSpeedingEventData, VuTimeAdjustmentData};
use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader, VuOverSpeedingControlData};
use crate::{Readable, Result};

#[derive(Debug)]
pub struct VuEvents {
    pub vu_fault_data: VuFaultData,
    pub vu_event_data: VuEventData,
    pub vu_over_speeding_control_data: VuOverSpeedingControlData,
    pub vu_over_speeding_event_data: VuOverSpeedingEventData,
    pub vu_time_adjustment_data: VuTimeAdjustmentData,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VuEvents> for VuEvents {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VuEvents> {
        debug!("VuEvents::from_data - Trep ID: {trep_id:?}");
        let vu_fault_data = VuFaultData::read(reader)?;
        let vu_event_data = VuEventData::read(reader)?;
        let vu_over_speeding_control_data = VuOverSpeedingControlData::read(reader)?;
        let vu_over_speeding_event_data = VuOverSpeedingEventData::read(reader)?;
        let vu_time_adjustment_data = VuTimeAdjustmentData::read(reader)?;

        let signature = Some(reader.read_into_vec(128)?);

        Ok(Self {
            vu_fault_data,
            vu_event_data,
            vu_over_speeding_control_data,
            vu_over_speeding_event_data,
            vu_time_adjustment_data,
            signature,
        })
    }
}
