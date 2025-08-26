use crate::Result;
use binary_data::{BinSeek, WriteBytes};

/// A trait for types that can be written to a binary stream.
pub trait Writable {
    /// Writes the binary representation of `self` to the given writer.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()>;
}
