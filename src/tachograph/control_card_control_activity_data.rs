use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug)]
pub struct ControlCardControlActivityDataParams {
    pub no_of_control_card_activity_records: u32,
}

impl ControlCardControlActivityDataParams {
    pub fn new(no_of_control_card_activity_records: u32) -> Self {
        Self { no_of_control_card_activity_records }
    }
}

#[derive(Debug)]
pub struct ControlCardControlActivityData<T> {
    pub control_card_pointer_newest_record: u16,
    pub control_card_activity_records: Vec<T>,
}

impl<T: Readable<T>> ReadableWithParams<ControlCardControlActivityData<T>> for ControlCardControlActivityData<T> {
    type P = ControlCardControlActivityDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<ControlCardControlActivityData<T>> {
        let control_card_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut control_card_activity_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_control_card_activity_records {
            let record = T::read(reader)?;
            control_card_activity_records.push(record);
        }
        Ok(Self { control_card_pointer_newest_record, control_card_activity_records })
    }
}
