use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, tacho::SpecificConditionRecord};

#[derive(Debug)]
pub struct SpecificConditionsParams {
    pub no_of_records: u8,
}

impl SpecificConditionsParams {
    pub fn new(no_of_records: u8) -> Self {
        Self { no_of_records }
    }
}

/// Information, stored in a driver card, a workshop card or a vehicle unit,
/// related to a specific condition (Annex 1C requirement 131, 277, 302, 329, and 356).
#[derive(Debug, Serialize)]
pub struct SpecificConditions {
    #[serde(rename = "conditionPointerNewestRecord")]
    pub condition_pointer_newest_record: u16,
    #[serde(rename = "specificConditionRecords")]
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}

impl ReadableWithParams<SpecificConditions> for SpecificConditions {
    type P = SpecificConditionsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<SpecificConditions> {
        let no_of_records = params.no_of_records;
        let mut specific_condition_records: Vec<SpecificConditionRecord> = Vec::new();
        let condition_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        for _ in 0..no_of_records {
            let specific_condition_record = SpecificConditionRecord::read(reader)?;
            if specific_condition_record.entry_time.has_data() {
                specific_condition_records.push(specific_condition_record);
            }
        }
        Ok(Self { condition_pointer_newest_record, specific_condition_records })
    }
}
