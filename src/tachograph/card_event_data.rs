use binary_data::{BinSeek, ReadBytes};

use crate::{ReadableWithParams, Result};

#[derive(Debug)]
pub struct CardEventDataParams {
    pub no_of_records: u8,
}

impl CardEventDataParams {
    pub fn new(no_of_records: u8) -> Self {
        Self { no_of_records }
    }
}

#[derive(Debug)]
pub struct CardEventData {
    pub no_of_records: u8,
}

impl ReadableWithParams<CardEventData> for CardEventData {
    type P = CardEventDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardEventData> {
        let no_of_records = params.no_of_records;
        // FIXME:
        Ok(Self { no_of_records })
    }
}
