use serde::Serialize;

use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug, Serialize)]
pub struct VUTransferResponseParameterItem<D> {
    pub type_id: VUTransferResponseParameterID,
    pub position: u32,
    pub data: D,
}
