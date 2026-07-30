use serde::Serialize;

use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VUTransferResponseParameterItem<D> {
    #[serde(rename = "typeId")]
    pub type_id: VUTransferResponseParameterID,
    pub position: u32,
    pub data: D,
}
