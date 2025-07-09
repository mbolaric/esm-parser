use binary_data::BigEndian;

use crate::{Readable, tacho::SpecificConditionRecord};

#[derive(Debug)]
pub struct VuSpecificConditionData {
    pub no_of_specific_condition_records: u16,
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
