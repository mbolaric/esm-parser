use crate::{Readable, tacho::TimeReal};

#[derive(Debug)]
pub struct VuDetailedSpeedBlock {
    pub speed_block_begin_date: TimeReal,
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
