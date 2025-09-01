//!
//! This module defines a trait for serializing data into a binary stream.
//!
//! The `Writable` trait provides a standardized way to write data structures to a
//! byte-oriented output stream. It is the counterpart to the `Readable` trait and is
//! essential for converting in-memory data representations into their binary format.
//!
//! The trait is designed to be generic over any writer that implements `binary_data::WriteBytes`
//! and `binary_data::BinSeek`, ensuring flexibility in handling different output destinations,
//! such as files, network sockets, or in-memory buffers.
//!
//! # `Writable` Trait
//!
//! The `Writable` trait defines a single method, `write`, which takes an immutable reference
//! to `self` and a mutable reference to a writer. Implementers of this trait are responsible
//! for writing their binary representation to the provided writer.
//!
//! # Examples
//!
//! ```
//! use binary_data::{BinMemoryBuffer, WriteBytes, BinSeek};
//! use esm_parser::Writable;
//! use esm_parser::Result;
//!
//! // A simple struct that can be written to a binary stream
//! struct MyData {
//!     field1: u8,
//!     field2: u16,
//! }
//!
//! impl Writable for MyData {
//!     fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
//!         writer.write_u8(self.field1)?;
//!         writer.write_u16_be(self.field2)?;
//!         Ok(())
//!     }
//! }
//!
//! // Example of writing the struct to a byte vector
//! let my_data = MyData { field1: 0x01, field2: 0x0203 };
//! let mut writer = BinMemoryBuffer::new();
//! my_data.write(&mut writer).unwrap();
//!
//! let bytes = writer.as_slice();
//! assert_eq!(bytes, &[0x01, 0x02, 0x03]);
//! ```

use crate::Result;
use binary_data::{BinSeek, WriteBytes};

/// A trait for types that can be serialized and written to a binary stream.
///
/// This trait is implemented by data structures that can be converted into a binary
/// representation and written to a writer that implements `WriteBytes` and `BinSeek`.
/// It is used for serializing data in a structured and reusable manner.
pub trait Writable {
    /// Writes the binary representation of `self` to the given writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - A mutable reference to a writer that implements `WriteBytes` and `BinSeek`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the write operation was successful. Returns `Ok(())`
    /// on success or an `Error` if writing fails.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()>;
}
