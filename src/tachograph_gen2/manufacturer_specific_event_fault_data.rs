use binary_data::{BinSeek, ReadBytes};

use crate::{Readable, Result};

#[derive(Debug)]
pub struct ManufacturerSpecificEventFaultData {
    pub manufacturer_code: u8,
    pub manufacturer_specific_error_code: Vec<u8>,
}

impl Readable<ManufacturerSpecificEventFaultData> for ManufacturerSpecificEventFaultData {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<ManufacturerSpecificEventFaultData> {
        let manufacturer_code = reader.read_u8()?;
        let manufacturer_specific_error_code = reader.read_into_vec(3)?;
        Ok(Self { manufacturer_code, manufacturer_specific_error_code })
    }
}
