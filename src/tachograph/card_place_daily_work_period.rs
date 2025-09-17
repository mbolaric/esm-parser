use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, tacho::TimeReal};

pub trait CardPlace {
    fn get_entry_time(&self) -> &TimeReal;
}

#[derive(Debug)]
pub struct CardPlaceDailyWorkPeriodParams {
    pub no_of_records: u32,
    pub size_of_newest_place_record: u8,
}

impl CardPlaceDailyWorkPeriodParams {
    pub fn new(no_of_records: u32, size_of_newest_place_record: u8) -> Self {
        Self { no_of_records, size_of_newest_place_record }
    }
}

#[derive(Debug, Serialize)]
pub struct CardPlaceDailyWorkPeriod<T> {
    pub place_pointer_newest_record: i32,
    pub place_records: Vec<T>,
}

impl<T: Readable<T> + CardPlace> ReadableWithParams<CardPlaceDailyWorkPeriod<T>> for CardPlaceDailyWorkPeriod<T> {
    type P = CardPlaceDailyWorkPeriodParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardPlaceDailyWorkPeriod<T>> {
        let place_pointer_newest_record = if params.size_of_newest_place_record == 1 {
            reader.read_u8()? as i32
        } else {
            reader.read_u16::<BigEndian>()? as i32
        };

        let mut place_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_records {
            let record = T::read(reader)?;
            if record.get_entry_time().has_data() {
                place_records.push(record);
            }
        }

        Ok(Self { place_pointer_newest_record, place_records })
    }
}
