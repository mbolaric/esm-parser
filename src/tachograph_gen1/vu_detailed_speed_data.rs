use binary_data::BigEndian;
use serde::Serialize;

use crate::{Readable, tacho::VuDetailedSpeedBlock};

/// Information, stored in a vehicle unit, related to the detailed speed of the vehicle.
#[derive(Debug, Serialize)]
pub struct VuDetailedSpeedData {
    #[serde(rename = "noOfSpeedBlocks")]
    pub no_of_speed_blocks: u16,
    #[serde(rename = "vuDetailedSpeedBlocks")]
    pub vu_detailed_speed_blocks: Vec<VuDetailedSpeedBlock>,
}

impl Readable<VuDetailedSpeedData> for VuDetailedSpeedData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDetailedSpeedData> {
        let no_of_speed_blocks = reader.read_u16::<BigEndian>()?;
        let mut vu_detailed_speed_blocks: Vec<VuDetailedSpeedBlock> = Vec::new();
        for _ in 0..no_of_speed_blocks {
            vu_detailed_speed_blocks.push(VuDetailedSpeedBlock::read(reader)?);
        }
        Ok(Self { no_of_speed_blocks, vu_detailed_speed_blocks })
    }
}
