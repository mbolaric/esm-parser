use binary_data::BigEndian;
use serde::Serialize;

use crate::tacho::SpecificConditionRecord;
use crate::{Readable, read_records};

/// Information, stored in a vehicle unit, related to specific conditions.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuSpecificConditionData"))]
pub struct VuSpecificConditionData {
    #[serde(rename = "noOfSpecificConditionRecords")]
    pub no_of_specific_condition_records: u16,
    #[serde(rename = "specificConditionRecords")]
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}

impl Readable for VuSpecificConditionData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuSpecificConditionData> {
        let no_of_specific_condition_records = reader.read_u16::<BigEndian>()?;
        let specific_condition_records =
            read_records(reader, no_of_specific_condition_records as usize, |reader| SpecificConditionRecord::read(reader))?;
        Ok(Self { no_of_specific_condition_records, specific_condition_records })
    }
}
