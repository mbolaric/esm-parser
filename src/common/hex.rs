//! This module provides helper utilities for converting byte slices into hexadecimal string representations.
//!
//! It introduces the `HexDisplay` trait, which extends types that can be referenced as byte slices (`AsRef<[u8]>`),
//! allowing them to be easily converted into hexadecimal strings. The module also provides a `HexHelper` struct
//! that wraps byte slices to facilitate the hexadecimal conversion.
//!
//! # Examples
//!
//! ```
//! use esm_parser::HexDisplay;
//!
//! let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
//!
//! // Convert to a lowercase hexadecimal string
//! let lower_hex = data.to_hex().to_lower_hex_string();
//! assert_eq!(lower_hex, "deadbeef");
//!
//! // Convert to an uppercase hexadecimal string
//! let upper_hex = data.to_hex().to_upper_hex_string();
//! assert_eq!(upper_hex, "DEADBEEF");
//!
//! // The `to_hex_string` trait method provides a direct way to get an uppercase hex string
//! let upper_hex_direct = data.to_hex_string();
//! assert_eq!(upper_hex_direct, "DEADBEEF");
//! ```

use std::fmt;

/// A helper struct that wraps a byte slice to provide hexadecimal string conversion.
///
/// This struct is created by the `to_hex` method in the `HexDisplay` trait.
pub struct HexHelper<'a>(&'a [u8]);

impl<'a> HexHelper<'a> {
    /// Creates a new `HexHelper` instance from a type that can be referenced as a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to a type that implements `AsRef<[u8]>`.
    fn new<T>(data: &'a T) -> HexHelper<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexHelper(data.as_ref())
    }

    /// Converts the byte slice to a lowercase hexadecimal string.
    pub fn to_lower_hex_string(&self) -> String {
        self.0.iter().map(|c| format!("{c:02x}")).collect()
    }

    /// Converts the byte slice to an uppercase hexadecimal string.
    pub fn to_upper_hex_string(&self) -> String {
        self.0.iter().map(|c| format!("{c:02X}")).collect()
    }
}

/// Implements the `Display` trait for `HexHelper` to format the byte slice as an uppercase hexadecimal string.
impl fmt::Display for HexHelper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:2X}")?;
        }
        Ok(())
    }
}

/// A trait that provides methods for converting a type to a hexadecimal string representation.
pub trait HexDisplay {
    /// Wraps the type in a `HexHelper` to allow for hexadecimal string conversion.
    fn to_hex(&self) -> HexHelper<'_>;

    /// Converts the type to an uppercase hexadecimal string.
    fn to_hex_string(&self) -> String;
}

/// Implements the `HexDisplay` trait for any type that can be referenced as a byte slice.
impl<T> HexDisplay for T
where
    T: ?Sized + AsRef<[u8]>,
{
    /// Creates a `HexHelper` for the given type.
    fn to_hex(&self) -> HexHelper<'_> {
        HexHelper::new(self)
    }

    /// Converts the type to an uppercase hexadecimal string by default.
    fn to_hex_string(&self) -> String {
        HexHelper::new(self).to_upper_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_hex_string() {
        let data1: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        assert_eq!(data1.to_hex().to_lower_hex_string(), "0123456789abcdef");

        let data2: &[u8] = &[0xFF, 0x00, 0xAA, 0x55];
        assert_eq!(data2.to_hex().to_lower_hex_string(), "ff00aa55");

        let data3: &[u8] = &[];
        assert_eq!(data3.to_hex().to_lower_hex_string(), "");
    }

    #[test]
    fn test_to_upper_hex_string() {
        let data1: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        assert_eq!(data1.to_hex().to_upper_hex_string(), "0123456789ABCDEF");

        let data2: &[u8] = &[0xFF, 0x00, 0xAA, 0x55];
        assert_eq!(data2.to_hex().to_upper_hex_string(), "FF00AA55");

        let data3: &[u8] = &[];
        assert_eq!(data3.to_hex().to_upper_hex_string(), "");
    }

    #[test]
    fn test_trait_to_hex_string() {
        // The trait method defaults to upper case
        let data1: &[u8] = &[0x01, 0x23, 0xAB, 0xCD];
        assert_eq!(data1.to_hex_string(), "0123ABCD");

        let vec1 = vec![0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(vec1.to_hex_string(), "DEADBEEF");
    }

    #[test]
    fn test_fmt_display() {
        let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
        let hex_display = data.to_hex();
        assert_eq!(format!("{}", hex_display), "DEADBEEF");

        let data_empty: &[u8] = &[];
        let hex_display_empty = data_empty.to_hex();
        assert_eq!(format!("{}", hex_display_empty), "");
    }
}
