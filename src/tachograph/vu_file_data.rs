use serde::Serialize;

use crate::tacho::VUTransferResponseParameterID;

pub type VUFilesList = Vec<VUFileData>;

pub trait VUDataFiles {
    fn get_data_files(&self) -> &VUFilesList;
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VUFileData {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    pub position: u32,
    pub size: u32,
    pub signature: Option<Vec<u8>>,
    pub signatures: Vec<Vec<u8>>,
    pub data: Option<Vec<u8>>,
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
}
