use binary_data::{BinSeek, ReadBytes};

use crate::Result;
use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader};

#[derive(Debug)]
pub struct VUActivity {}

impl VUTransferResponseParameterReader<VUActivity> for VUActivity {
    fn from_data<R: ReadBytes + BinSeek>(_trep_id: VUTransferResponseParameterID, _reader: &mut R) -> Result<VUActivity> {
        // FIXME:
        Ok(Self {})
    }
}
