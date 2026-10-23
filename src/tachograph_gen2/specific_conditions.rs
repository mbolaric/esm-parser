use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::SpecificConditionRecord;
use crate::tachograph_gen2::card_records::read_card_records;
use crate::{Error, ReadableWithParams, Result};

#[derive(Debug)]
pub struct SpecificConditionsParams {
    pub no_of_records: u32,
}

impl SpecificConditionsParams {
    pub fn new(no_of_records: u32) -> Self {
        Self { no_of_records }
    }
}

/// Information, stored in a driver card, a workshop card or a vehicle unit,
/// related to a specific condition (Annex 1C requirement 131, 277, 302, 329, and 356).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SpecificConditions"))]
pub struct SpecificConditions {
    #[serde(rename = "conditionPointerNewestRecord")]
    pub condition_pointer_newest_record: u16,
    #[serde(rename = "specificConditionRecords")]
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}

impl ReadableWithParams for SpecificConditions {
    type P = SpecificConditionsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<SpecificConditions> {
        if params.no_of_records > 112 {
            return Err(Error::RecordOutOfRange(format!("Specific-condition capacity: {}", params.no_of_records)));
        }
        let (condition_pointer_newest_record, records) =
            read_card_records::<_, SpecificConditionRecord>(reader, params.no_of_records, 5, "Specific conditions")?;
        let specific_condition_records = records.into_iter().filter(|record| record.entry_time.has_data()).collect();

        Ok(Self { condition_pointer_newest_record, specific_condition_records })
    }
}
