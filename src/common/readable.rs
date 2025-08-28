use binary_data::{BinSeek, ReadBytes};

use crate::Result;

/// A trait for types that can read from a binary stream.
pub trait Readable<T> {
    fn read<R: ReadBytes + BinSeek>(_reader: &mut R) -> Result<T>;
}

/// A trait for types that can read from a binary stream and provide parameters.
pub trait ReadableWithParams<T> {
    type P;
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<T>;
}
