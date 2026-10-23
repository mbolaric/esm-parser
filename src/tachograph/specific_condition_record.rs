use serde::Serialize;

use crate::Readable;
use crate::tacho::{RecordType, SpecificConditionType, TimeReal, VuRecordLayout};

/// Information, stored in a driver card, a workshop card or a vehicle unit,
/// related to a specific condition (requirements Annex 1C 130, 276, 301, 328, and 355).
#[derive(Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct SpecificConditionRecord {
    #[serde(rename = "entryTime")]
    pub entry_time: TimeReal,
    #[serde(rename = "specificConditionType")]
    pub specific_condition_type: SpecificConditionType,
}

impl Readable for SpecificConditionRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SpecificConditionRecord> {
        let entry_time = TimeReal::read(reader)?;
        let specific_condition_type: SpecificConditionType = reader.read_u8()?.into();
        Ok(Self { entry_time, specific_condition_type })
    }
}

impl VuRecordLayout for SpecificConditionRecord {
    const RECORD_TYPE: RecordType = RecordType::SpecificConditionRecord;
    const RECORD_SIZE: u16 = 5;
}
