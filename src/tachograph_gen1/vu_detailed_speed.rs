use serde::Serialize;

use crate::{Readable, gen1::VuDetailedSpeedData};

#[derive(Debug, Serialize)]
pub struct VuDetailedSpeed {
    pub vu_detailed_speed_data: VuDetailedSpeedData,
    pub signature: Option<Vec<u8>>,
}

impl Readable<VuDetailedSpeed> for VuDetailedSpeed {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuDetailedSpeed> {
        let vu_detailed_speed_data = VuDetailedSpeedData::read(reader)?;
        let signature = Some(reader.read_into_vec(128)?);
        Ok(Self { vu_detailed_speed_data, signature })
    }
}
