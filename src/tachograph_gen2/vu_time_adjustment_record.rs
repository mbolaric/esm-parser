use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{Address, Name, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct VuTimeAdjustmentRecord {
    pub old_time_value: TimeReal,
    pub new_time_value: TimeReal,
    pub workshop_name: Name,
    pub workshop_address: Address,
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
