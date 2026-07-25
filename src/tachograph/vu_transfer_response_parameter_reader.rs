use binary_data::{BinSeek, ReadBytes};

use crate::Result;
use crate::tacho::VUTransferResponseParameterID;

pub trait VUTransferResponseParameterReader<T> {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<T>;
}
