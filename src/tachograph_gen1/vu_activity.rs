use binary_data::{BinSeek, ReadBytes};

use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader};
use crate::Result;

#[derive(Debug)]
pub struct VUActivity {}

impl VUTransferResponseParameterReader<VUActivity> for VUActivity {
    fn from_data<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUActivity> {
        Ok(Self {})
    }
}
