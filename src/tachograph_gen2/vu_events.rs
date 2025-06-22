use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen2::DataInfo;
use crate::tacho::VUTransferResponseParameterID;
use crate::Result;

#[derive(Debug)]
pub struct VUEvents {}

impl VUEvents {
    pub fn from_data<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUEvents> {
        debug!("VUEvents::from_data - {:?}", trep_id);
        let fault = DataInfo::read(reader, trep_id.clone())?;
        let event = DataInfo::read(reader, trep_id.clone())?;
        let over_speeding_control_data = DataInfo::read(reader, trep_id.clone())?;
        let over_speeding_event = DataInfo::read(reader, trep_id.clone())?;
        let time_adjustment = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self {})
    }
}
