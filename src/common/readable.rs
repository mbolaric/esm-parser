use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};

use crate::{Error, Result};

pub(crate) fn ensure_consumed<R: BinSeek>(reader: &mut R, name: &str) -> Result<()> {
    let position = reader.pos()?;
    let length = reader.len()?;
    if position != length {
        return Err(Error::InvalidDataParse(format!("{name}: consumed {position} of {length} bytes")));
    }
    Ok(())
}

pub(crate) fn validate_ring_pointer(pointer: u32, capacity: u32, name: &str) -> Result<()> {
    if capacity == 0 || capacity > u32::from(u16::MAX) || pointer >= capacity {
        return Err(Error::RecordOutOfRange(format!("{name}: pointer {pointer}, capacity {capacity}")));
    }
    Ok(())
}

/// Keeps a record parser inside its physical slot and rejects unconsumed bytes.
pub(crate) fn read_records_with_size<R, T, F>(reader: &mut R, count: usize, size: usize, mut read_record: F) -> Result<Vec<T>>
where
    R: ReadBytes + BinSeek,
    F: FnMut(&mut BinMemoryBuffer) -> Result<T>,
{
    let length = count.checked_mul(size).ok_or_else(|| Error::InvalidDataParse("Record array size overflow".to_owned()))?;
    if size == 0 || length > reader.len()?.saturating_sub(reader.pos()?) {
        return Err(Error::InvalidDataParse("Invalid record size or truncated record array".to_owned()));
    }
    read_records(reader, count, |reader| {
        let mut slot = BinMemoryBuffer::from(reader.read_into_vec(
            u32::try_from(size).map_err(|_| Error::InvalidDataParse("Record size exceeds the binary format limit".to_owned()))?,
        )?);
        let record = read_record(&mut slot)?;
        ensure_consumed(&mut slot, "Record")?;
        Ok(record)
    })
}

/// Reads exactly `count` records using the supplied record parser.
pub(crate) fn read_records<R, T, F>(reader: &mut R, count: usize, mut read_record: F) -> Result<Vec<T>>
where
    R: ReadBytes + BinSeek,
    F: FnMut(&mut R) -> Result<T>,
{
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(read_record(reader)?);
    }
    Ok(records)
}

/// A trait for types that can be deserialized from a binary stream.
/// The trait are designed to work with any reader that implements `binary_data::ReadBytes`
/// and `binary_data::BinSeek`.
///
/// This trait is implemented by data structures that can be read directly from a reader
/// that implements `ReadBytes` and `BinSeek`. It is used for self-contained data structures
/// that do not require external information for deserialization.
///
/// # Examples
///
/// ```
/// use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
/// use esm_parser::{Readable, Result};
///
/// // A simple struct that can be read from a binary stream
/// struct MyData {
///     field1: u8,
///     field2: u16,
/// }
///
/// impl Readable for MyData {
///     fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
///         let field1 = reader.read_u8()?;
///         let field2 = reader.read_u16::<BigEndian>()?;
///         Ok(Self { field1, field2 })
///     }
/// }
///
/// // Example of reading the struct from a byte slice
/// let buffer: &[u8] = &[0x01, 0x02, 0x03];
/// let mut reader = BinMemoryBuffer::from(buffer);
/// let my_data = MyData::read(&mut reader).unwrap();
/// assert_eq!(my_data.field1, 0x01);
/// assert_eq!(my_data.field2, 0x0203);
/// ```
pub trait Readable: Sized {
    /// Reads an instance of the type from the given reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `ReadBytes` and `BinSeek`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if reading fails.
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self>;
}

/// A trait for types that can be deserialized from a binary stream using additional parameters.
/// The trait are designed to work with any reader that implements `binary_data::ReadBytes`
/// and `binary_data::BinSeek`.
///
/// This trait is an extension of `Readable` for data structures whose deserialization depends
/// on external context or parameters. The `P` associated type defines the structure of these
/// parameters.
pub trait ReadableWithParams: Sized {
    /// The type of the parameters required for deserialization.
    type P;

    /// Reads an instance of the type from the given reader using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `ReadBytes` and `BinSeek`.
    /// * `params` - A reference to the parameters required for deserialization.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if reading fails.
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_boundaries_reject_underreads_and_overreads_without_borrowing_from_the_next_slot() {
        for size in [3, 5] {
            let mut reader = BinMemoryBuffer::from(vec![0; 8]);
            assert!(read_records_with_size(&mut reader, 2, 4, |slot| Ok(slot.read_into_vec(size)?)).is_err());
            assert_eq!(reader.pos().unwrap(), 4);
        }
    }
}
