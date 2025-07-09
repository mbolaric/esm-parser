use crate::{
    Readable,
    tacho::{SpecificCondition, TimeReal},
};

#[derive(Debug)]
pub struct SpecificConditionRecord {
    pub entry_time: TimeReal,
    pub specific_condition_type: SpecificCondition,
}

impl Readable<SpecificConditionRecord> for SpecificConditionRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SpecificConditionRecord> {
        let entry_time = TimeReal::read(reader)?;
        let specific_condition_type: SpecificCondition = reader.read_u8()?.into();
        Ok(Self { entry_time, specific_condition_type })
    }
}
