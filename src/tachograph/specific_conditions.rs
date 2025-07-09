use binary_data::{BinSeek, ReadBytes};

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

#[derive(Debug)]
pub struct SpecificConditions {
    pub specific_conditions: Vec<SpecificConditionRecord>,
}

impl ReadableWithParams<SpecificConditions> for SpecificConditions {
    type P = SpecificConditionsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<SpecificConditions> {
        let no_of_records = params.no_of_records;
        let mut specific_conditions: Vec<SpecificConditionRecord> = Vec::new();
        for _ in 0..no_of_records {
            let specific_condition_record = SpecificConditionRecord::read(reader)?;
            if specific_condition_record.entry_time.has_data() {
                specific_conditions.push(specific_condition_record);
            }
        }
        Ok(Self { specific_conditions })
    }
}
