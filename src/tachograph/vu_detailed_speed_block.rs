use serde::Serialize;

use crate::{Readable, tacho::TimeReal};

/// Information, stored in a vehicle unit, related to the vehicle's detailed
/// speed for a minute during which the vehicle has been moving
/// (Annnex 1B requirement 093 and Annex 1C requirement 116).
#[derive(Debug, Serialize)]
pub struct VuDetailedSpeedBlock {
    #[serde(rename = "speedBlockBeginDate")]
    pub speed_block_begin_date: TimeReal,
    #[serde(rename = "speedsPerSecond")]
    pub speeds_per_second: Vec<u8>,
}

impl Readable<VuDetailedSpeedBlock> for VuDetailedSpeedBlock {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDetailedSpeedBlock> {
        let speed_block_begin_date = TimeReal::read(reader)?;
        let mut speeds_per_second: Vec<u8> = Vec::new();
        for _ in 0..60 {
            speeds_per_second.push(reader.read_u8()?);
        }
        Ok(Self { speed_block_begin_date, speeds_per_second })
    }
}
