use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{ControlType, TimeReal},
};

/// Information, stored in a vehicle unit, related to a control performed using
/// this VU (Annex 1B requirement 102 and Annex 1C requirement 126).
#[derive(Debug, Serialize)]
pub struct VuControlActivityRecord {
    #[serde(rename = "controlType")]
    pub control_type: ControlType,
    #[serde(rename = "controlTime")]
    pub control_time: TimeReal,
    #[serde(rename = "controlCardNumberAndGeneration")]
    pub control_card_number_and_generation: FullCardNumberAndGeneration,
    #[serde(rename = "downloadPeriodBeginTime")]
    pub download_period_begin_time: TimeReal,
    #[serde(rename = "downloadPeriodEndTime")]
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
