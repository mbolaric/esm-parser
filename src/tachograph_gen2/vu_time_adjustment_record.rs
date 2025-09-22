use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{Address, Name, TimeReal},
};

/// Information, stored in a vehicle unit, related a time adjustment
/// performed outside the frame of a regular calibration (Annex 1B
/// requirement 101 and Annex 1C requirement 124 and 125).
#[derive(Debug, Serialize)]
pub struct VuTimeAdjustmentRecord {
    #[serde(rename = "oldTimeValue")]
    pub old_time_value: TimeReal,
    #[serde(rename = "newTimeValue")]
    pub new_time_value: TimeReal,
    #[serde(rename = "workshopName")]
    pub workshop_name: Name,
    #[serde(rename = "workshopAddress")]
    pub workshop_address: Address,
    #[serde(rename = "workshopCardNumberAndGeneration")]
    pub workshop_card_number_and_generation: FullCardNumberAndGeneration,
}

impl Readable<VuTimeAdjustmentRecord> for VuTimeAdjustmentRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuTimeAdjustmentRecord> {
        let old_time_value = TimeReal::read(reader)?;
        let new_time_value = TimeReal::read(reader)?;
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;

        Ok(Self { old_time_value, new_time_value, workshop_name, workshop_address, workshop_card_number_and_generation })
    }
}
