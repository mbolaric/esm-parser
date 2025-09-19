use serde::Serialize;

use crate::{
    Readable,
    tacho::{SpecificConditionType, TimeReal},
};

/// Information, stored in a driver card, a workshop card or a vehicle unit,
/// related to a specific condition (requirements Annex 1C 130, 276, 301, 328, and 355).
#[derive(Debug, Serialize)]
pub struct SpecificConditionRecord {
    #[serde(rename = "entryTime")]
    pub entry_time: TimeReal,
    #[serde(rename = "specificConditionType")]
    pub specific_condition_type: SpecificConditionType,
}

impl Readable<SpecificConditionRecord> for SpecificConditionRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SpecificConditionRecord> {
        let entry_time = TimeReal::read(reader)?;
        let specific_condition_type: SpecificConditionType = reader.read_u8()?.into();
        Ok(Self { entry_time, specific_condition_type })
    }
}
