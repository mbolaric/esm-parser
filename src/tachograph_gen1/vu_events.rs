use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::gen1::{VuEventData, VuFaultData, VuOverSpeedingEventData, VuTimeAdjustmentData};
use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader, VuOverSpeedingControlData};
use crate::{Readable, Result};

const SIGNATURE_LENGTH: u32 = 128;

/// Data structure generation 1,
#[derive(Debug, Serialize)]
pub struct VuEvents {
    #[serde(rename = "vuFaultData")]
    pub vu_fault_data: VuFaultData,
    #[serde(rename = "vuEventData")]
    pub vu_event_data: VuEventData,
    #[serde(rename = "vuOverSpeedingControlData")]
    pub vu_over_speeding_control_data: VuOverSpeedingControlData,
    #[serde(rename = "vuOverSpeedingEventData")]
    pub vu_over_speeding_event_data: VuOverSpeedingEventData,
    #[serde(rename = "vuTimeAdjustmentData")]
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

        let signature = Some(reader.read_into_vec(SIGNATURE_LENGTH)?);

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
