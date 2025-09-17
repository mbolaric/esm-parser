use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{ControlType, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct VuControlActivityRecord {
    pub control_type: ControlType,
    pub control_time: TimeReal,
    pub control_card_number_and_generation: FullCardNumberAndGeneration,
    pub download_period_begin_time: TimeReal,
    pub download_period_end_time: TimeReal,
}

impl Readable<VuControlActivityRecord> for VuControlActivityRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuControlActivityRecord> {
        let control_type: ControlType = reader.read_u8()?.into();
        let control_time = TimeReal::read(reader)?;
        let control_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let download_period_begin_time = TimeReal::read(reader)?;
        let download_period_end_time = TimeReal::read(reader)?;
        Ok(Self {
            control_type,
            control_time,
            control_card_number_and_generation,
            download_period_begin_time,
            download_period_end_time,
        })
    }
}
