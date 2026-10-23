use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{Error, Readable, Result, read_records_with_size};

/// Reads a Gen2 card EF's two-byte slot pointer and fixed-size records, retaining every physical slot.
pub(super) fn read_card_records<R: ReadBytes + BinSeek, T: Readable>(
    reader: &mut R,
    count: u32,
    record_size: usize,
    name: &str,
) -> Result<(u16, Vec<T>)> {
    if count == 0 || count > u32::from(u16::MAX) {
        return Err(Error::RecordOutOfRange(format!("{name} record count: {count}")));
    }
    let expected_size = 2 + count as usize * record_size;
    let remaining = reader.len()?.saturating_sub(reader.pos()?);
    if remaining != expected_size {
        return Err(Error::InvalidDataParse(format!(
            "{name} EF size: expected {expected_size} bytes for {count} records, got {remaining}"
        )));
    }
    let pointer = reader.read_u16::<BigEndian>()?;
    if u32::from(pointer) >= count {
        return Err(Error::RecordOutOfRange(format!("{name} newest-record pointer: {pointer}, capacity: {count}")));
    }
    let records = read_records_with_size(reader, count as usize, record_size, T::read)?;
    if reader.pos()? != reader.len()? {
        return Err(Error::InvalidDataParse(format!("{name} record parser did not consume the complete EF")));
    }
    Ok((pointer, records))
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;
    use crate::tacho::TimeReal;

    #[test]
    fn rejects_a_record_size_that_does_not_match_the_parser() {
        let mut reader = BinMemoryBuffer::from(vec![0; 12]);
        assert!(read_card_records::<_, TimeReal>(&mut reader, 2, 5, "Test records").is_err());
        let mut reader = BinMemoryBuffer::from(vec![0; 10]);
        assert_eq!(read_card_records::<_, TimeReal>(&mut reader, 2, 4, "Test records").unwrap().1.len(), 2);
    }
}
