use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result};

/// Manufacturer specific error codes simplify the error analysis and main-tenance
/// of vehicle units.
#[derive(Debug, Serialize)]
pub struct ManufacturerSpecificEventFaultData {
    #[serde(rename = "manufacturerCode")]
    pub manufacturer_code: u8,
    #[serde(rename = "manufacturerSpecificErrorCode")]
    pub manufacturer_specific_error_code: Vec<u8>,
}

impl Readable<ManufacturerSpecificEventFaultData> for ManufacturerSpecificEventFaultData {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<ManufacturerSpecificEventFaultData> {
        let manufacturer_code = reader.read_u8()?;
        let manufacturer_specific_error_code = reader.read_into_vec(3)?;
        Ok(Self { manufacturer_code, manufacturer_specific_error_code })
    }
}
