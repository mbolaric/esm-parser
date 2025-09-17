use serde::Serialize;

use crate::{
    Readable,
    tacho::{SpecificConditionType, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct SpecificConditionRecord {
    pub entry_time: TimeReal,
    pub specific_condition_type: SpecificConditionType,
}

impl Readable<SpecificConditionRecord> for SpecificConditionRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SpecificConditionRecord> {
        let entry_time = TimeReal::read(reader)?;
        let specific_condition_type: SpecificConditionType = reader.read_u8()?.into();
        Ok(Self { entry_time, specific_condition_type })
    }
}
