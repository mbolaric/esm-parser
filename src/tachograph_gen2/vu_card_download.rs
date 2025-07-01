use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::DataInfo;
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUCardDownload {}

impl VUCardDownload {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUCardDownload> {
        debug!("VUCardDownload::from_data - Trep ID: {:?}", trep_id);
        let card = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self {})
    }
}
