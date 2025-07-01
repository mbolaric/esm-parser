use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::DataInfo;
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUSpeed {}

impl VUSpeed {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUSpeed> {
        debug!("VUSpeed::from_data - Trep ID: {:?}", trep_id);
        let detailed_speed = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self {})
    }
}
