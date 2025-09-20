use binary_data::BigEndian;
use serde::Serialize;

use crate::{Readable, tacho::SpecificConditionRecord};

/// Information, stored in a vehicle unit, related to specific conditions.
#[derive(Debug, Serialize)]
pub struct VuSpecificConditionData {
    #[serde(rename = "noOfSpecificConditionRecords")]
    pub no_of_specific_condition_records: u16,
    #[serde(rename = "specificConditionRecords")]
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}

impl Readable<VuSpecificConditionData> for VuSpecificConditionData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuSpecificConditionData> {
        let no_of_specific_condition_records = reader.read_u16::<BigEndian>()?;
        let mut specific_condition_records: Vec<SpecificConditionRecord> =
            Vec::with_capacity(no_of_specific_condition_records as usize);
        for _ in 0..no_of_specific_condition_records {
            let record = SpecificConditionRecord::read(reader)?;
            specific_condition_records.push(record);
        }
        Ok(Self { no_of_specific_condition_records, specific_condition_records })
    }
}
