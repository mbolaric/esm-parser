use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUTransferResponseParameterItem<D> {
    pub type_id: VUTransferResponseParameterID,
    pub position: u32,
    pub data: D,
}
