use crate::tacho::{VUFileData, VUTransferResponseParameterID};

pub trait VUTransferResponseParameter {
    fn is_oddball_crash_dump(&self) -> bool;

    fn extract_file_data(
        &self,
        _trep_id: VUTransferResponseParameterID,
        _position: u32,
        _raw_bytes: &[u8],
    ) -> Option<VUFileData> {
        None
    }
}
