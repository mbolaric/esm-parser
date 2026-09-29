use serde::Serialize;

use crate::tacho::TimeReal;
use crate::{Readable, read_records};

/// Information, stored in a vehicle unit, related to the vehicle's detailed
/// speed for a minute during which the vehicle has been moving
/// (Annnex 1B requirement 093 and Annex 1C requirement 116).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuDetailedSpeedBlock {
    #[serde(rename = "speedBlockBeginDate")]
    pub speed_block_begin_date: TimeReal,
    #[serde(rename = "speedsPerSecond")]
    pub speeds_per_second: Vec<u8>,
}

impl Readable for VuDetailedSpeedBlock {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDetailedSpeedBlock> {
        let speed_block_begin_date = TimeReal::read(reader)?;
        let speeds_per_second = read_records(reader, 60, |reader| Ok(reader.read_u8()?))?;
        Ok(Self { speed_block_begin_date, speeds_per_second })
    }
}
