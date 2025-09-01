//!
//! This module defines traits for deserializing data from a binary stream.
//!
//! It provides two main traits, `Readable` and `ReadableWithParams`, which are used
//! throughout the parser to abstract the process of reading and constructing data structures
//! from a byte-oriented input source. These traits are fundamental to the modular and
//! type-safe design of the data parsing logic.
//!
//! The traits are designed to work with any reader that implements `binary_data::ReadBytes`
//! and `binary_data::BinSeek`.
//!
//! # `Readable` Trait
//!
//! The `Readable` trait is for types that can be deserialized without any external parameters.
//! It defines a single method, `read`, which takes a mutable reference to a reader and returns
//! an instance of the type.
//!
//! # `ReadableWithParams` Trait
//!
//! The `ReadableWithParams` trait is for types that require additional context or parameters
//! for deserialization. This is useful when the structure or interpretation of the data
//! depends on previously parsed information.
//!
//! # Examples
//!
//! ```
//! use binary_data::{BinMemoryBuffer, ReadBytes, BinSeek};
//! use esm_parser::Readable;
//! use esm_parser::Result;
//!
//! // A simple struct that can be read from a binary stream
//! struct MyData {
//!     field1: u8,
//!     field2: u16,
//! }
//!
//! impl Readable<Self> for MyData {
//!     fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
//!         let field1 = reader.read_u8()?;
//!         let field2 = reader.read_u16_be()?;
//!         Ok(Self { field1, field2 })
//!     }
//! }
//!
//! // Example of reading the struct from a byte slice
//! let mut reader = BinMemoryBuffer::from(&[0x01, 0x02, 0x03]);
//! let my_data = MyData::read(&mut reader).unwrap();
//! assert_eq!(my_data.field1, 0x01);
//! assert_eq!(my_data.field2, 0x0203);
//! ```

use binary_data::{BinSeek, ReadBytes};

use crate::Result;

/// A trait for types that can be deserialized from a binary stream.
///
/// This trait is implemented by data structures that can be read directly from a reader
/// that implements `ReadBytes` and `BinSeek`. It is used for self-contained data structures
/// that do not require external information for deserialization.
pub trait Readable<T> {
    /// Reads an instance of the type from the given reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `ReadBytes` and `BinSeek`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized type `T` or an `Error` if reading fails.
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<T>;
}

/// A trait for types that can be deserialized from a binary stream using additional parameters.
///
/// This trait is an extension of `Readable` for data structures whose deserialization depends
/// on external context or parameters. The `P` associated type defines the structure of these
/// parameters.
pub trait ReadableWithParams<T> {
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
    /// A `Result` containing the deserialized type `T` or an `Error` if reading fails.
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<T>;
}
