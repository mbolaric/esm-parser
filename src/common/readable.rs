use binary_data::{BinSeek, ReadBytes};

use crate::Result;

pub trait Readable<T> {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<T>;
}
