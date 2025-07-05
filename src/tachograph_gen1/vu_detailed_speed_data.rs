use binary_data::BigEndian;

use crate::{Readable, tacho::VuDetailedSpeedBlock};

#[derive(Debug)]
pub struct VuDetailedSpeedData {
    pub no_of_speed_blocks: u16,
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
