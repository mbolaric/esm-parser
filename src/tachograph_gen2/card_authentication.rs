use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{Error, Readable, Result, read_records};

#[derive(Debug)]
pub struct CardAuthenticationRecordsParams {
    pub no_of_records: u32,
}

impl CardAuthenticationRecordsParams {
    pub fn new(no_of_records: u32) -> Self {
        Self { no_of_records }
    }
}

pub(super) fn read_card_authentication_records<R: ReadBytes + BinSeek, T: Readable>(
    reader: &mut R,
    params: &CardAuthenticationRecordsParams,
) -> Result<(u16, Vec<T>)> {
    let count = params.no_of_records;
    if count == 0 || count > u32::from(u16::MAX) {
        return Err(Error::RecordOutOfRange(format!("Card authentication record count: {count}")));
    }
    let expected_size = 2 + count as usize * 5;
    let remaining = reader.len()?.saturating_sub(reader.pos()?);
    if remaining != expected_size {
        return Err(Error::InvalidDataParse(format!(
            "Card authentication EF size: expected {expected_size} bytes for {count} records, got {remaining}"
        )));
    }
    let pointer = reader.read_u16::<BigEndian>()?;
    if u32::from(pointer) >= count {
        return Err(Error::RecordOutOfRange(format!("Card authentication newest-record pointer: {pointer}, capacity: {count}")));
    }
    let records = read_records(reader, count as usize, T::read)?;
    Ok((pointer, records))
}
