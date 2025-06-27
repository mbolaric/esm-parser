use binary_data::{BinSeek, ReadBytes};

use crate::{Error, Result};

pub trait Readable<T> {
    fn read<R: ReadBytes + BinSeek>(_reader: &mut R) -> Result<T> {
        Err(Error::NotImplemented)
    }
}

pub trait ReadableWithParams<T> {
    type P;
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<T>;
}
