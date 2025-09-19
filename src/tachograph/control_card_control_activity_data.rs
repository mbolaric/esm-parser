use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

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

/// Information, stored in a control card, related to control activity
/// performed with the card (Annex 1C requirement 361 and 367).
#[derive(Debug, Serialize)]
pub struct ControlCardControlActivityData<T> {
    #[serde(rename = "controlPointerNewestRecord")]
    pub control_pointer_newest_record: u16,
    #[serde(rename = "controlActivityRecords")]
    pub control_activity_records: Vec<T>,
}

impl<T: Readable<T>> ReadableWithParams<ControlCardControlActivityData<T>> for ControlCardControlActivityData<T> {
    type P = ControlCardControlActivityDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<ControlCardControlActivityData<T>> {
        let control_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut control_activity_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_control_card_activity_records {
            let record = T::read(reader)?;
            control_activity_records.push(record);
        }
        Ok(Self { control_pointer_newest_record, control_activity_records })
    }
}
