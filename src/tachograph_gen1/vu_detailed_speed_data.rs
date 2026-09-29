use binary_data::BigEndian;
use serde::Serialize;

use crate::tacho::VuDetailedSpeedBlock;
use crate::{Readable, read_records};

/// Information, stored in a vehicle unit, related to the detailed speed of the vehicle.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuDetailedSpeedData"))]
pub struct VuDetailedSpeedData {
    #[serde(rename = "noOfSpeedBlocks")]
    pub no_of_speed_blocks: u16,
    #[serde(rename = "vuDetailedSpeedBlocks")]
    pub vu_detailed_speed_blocks: Vec<VuDetailedSpeedBlock>,
}

impl Readable for VuDetailedSpeedData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDetailedSpeedData> {
        let no_of_speed_blocks = reader.read_u16::<BigEndian>()?;
        let vu_detailed_speed_blocks =
            read_records(reader, no_of_speed_blocks as usize, |reader| VuDetailedSpeedBlock::read(reader))?;
        Ok(Self { no_of_speed_blocks, vu_detailed_speed_blocks })
    }
}
