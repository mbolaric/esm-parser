use binary_data::{BinSeek, ReadBytes};

use crate::tacho::VUTransferResponseParameterID;
use crate::Result;

pub trait VUTransferResponseParameterReader<T> {
    fn from_data<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<T>;
}
