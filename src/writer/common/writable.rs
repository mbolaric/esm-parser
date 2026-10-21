use std::sync::Arc;

use binary_data::{BinMemoryBuffer, BinSeek, WriteBytes};

use super::string_encoding::{encode_code_page_padded, encode_ia5_padded};
use super::validatable::Validatable;
use crate::{CodePage, Result};

/// A closure that generates a digital signature for an arbitrary byte payload.
pub type SignatureSigner = Arc<dyn Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync>;

/// Specifies how signatures should be handled when serializing tachograph data.
#[derive(Clone, Default)]
pub enum SignaturePolicy {
    /// Preserves the existing signature if present (Default).
    /// Suitable for data downloaded directly from tachographs or smart card readers.
    #[default]
    Keep,
    /// Omits signature elementary files (Cards) and signature blocks (VUs).
    Strip,
    /// Uses provided signature bytes to replace existing signatures.
    Custom(Vec<u8>),
    /// Dynamically computes signatures using a signing closure (e.g. using a private RSA / ECC key).
    Sign(SignatureSigner),
}

/// Options configuring the binary serialization process.
#[derive(Clone, Default)]
pub struct WriteOptions {
    pub signature_policy: SignaturePolicy,
}

impl WriteOptions {
    /// Creates options that preserve existing signatures.
    pub fn keep_signatures() -> Self {
        Self { signature_policy: SignaturePolicy::Keep }
    }

    /// Creates options that omit/strip all signatures.
    pub fn strip_signatures() -> Self {
        Self { signature_policy: SignaturePolicy::Strip }
    }

    /// Creates options that replace signatures with custom bytes.
    pub fn with_custom_signature(signature: Vec<u8>) -> Self {
        Self { signature_policy: SignaturePolicy::Custom(signature) }
    }

    /// Creates options that compute signatures dynamically using a signing closure.
    pub fn with_signer<F>(signer: F) -> Self
    where
        F: Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static,
    {
        Self { signature_policy: SignaturePolicy::Sign(Arc::new(signer)) }
    }
}

/// Writes an iterator/slice of records using the supplied record writer.
pub fn write_records<W, T, F>(writer: &mut W, records: &[T], mut write_record: F) -> Result<()>
where
    W: WriteBytes + BinSeek,
    F: FnMut(&mut W, &T) -> Result<()>,
{
    for record in records {
        write_record(writer, record)?;
    }
    Ok(())
}

/// Writes a 1-byte `CodePage` followed by an `N`-byte space-padded string encoded in `code_page`.
pub fn write_code_page_string<const N: usize, W: WriteBytes + BinSeek>(
    writer: &mut W,
    field: &str,
    code_page: &CodePage,
    s: &str,
) -> Result<()> {
    let padded = encode_code_page_padded::<N>(field, s, code_page)?;
    writer.write_u8(*code_page as u8)?;
    writer.write_all(&padded)?;
    Ok(())
}

/// Writes an `N`-byte space-padded IA5 (ASCII) string.
pub fn write_ia5_string<const N: usize, W: WriteBytes + BinSeek>(writer: &mut W, field: &str, s: &str) -> Result<()> {
    let padded = encode_ia5_padded::<N>(field, s)?;
    writer.write_all(&padded)?;
    Ok(())
}
/// A trait for types that can be validated and serialized to a binary stream.
///
/// Every `Writable` type must also implement [`Validatable`]. Calling [`Writable::write`],
/// [`Writable::write_with_options`], or [`Writable::to_bytes`] automatically validates the
/// entire value tree upfront before emitting any bytes.
/// The zero-byte guarantee covers upfront validation failure only. Raw methods
/// bypass validation; I/O or later serialization failures may leave partial output.
///
/// Implementors define [`Writable::write_raw`] (and optionally [`Writable::write_raw_with_options`]),
/// and should call `write_raw` on nested child structs to avoid redundant re-validation.
///
/// # Examples
///
/// ```
/// use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, WriteBytes};
/// use esm_parser::Result;
/// use esm_parser::writer::{Validatable, Writable};
///
/// struct MyData {
///     field1: u8,
///     field2: u16,
/// }
///
/// impl Validatable for MyData {
///     fn validate(&self) -> Result<()> {
///         Ok(())
///     }
/// }
///
/// impl Writable for MyData {
///     fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
///         writer.write_u8(self.field1)?;
///         writer.write_u16::<BigEndian>(self.field2)?;
///         Ok(())
///     }
/// }
///
/// let my_data = MyData { field1: 0x01, field2: 0x0203 };
/// let mut writer = BinMemoryBuffer::new();
/// my_data.write(&mut writer).unwrap();
///
/// assert_eq!(writer.as_slice(), &[0x01, 0x02, 0x03]);
/// ```
pub trait Writable: Validatable {
    /// Writes the binary representation of `self` without re-running validation.
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()>;

    /// Writes the binary representation of `self` with explicit options without re-running validation.
    fn write_raw_with_options<W: WriteBytes + BinSeek>(&self, writer: &mut W, _options: &WriteOptions) -> Result<()> {
        self.write_raw(writer)
    }

    /// Validates `self` upfront and writes its binary representation to `writer` using default options.
    ///
    /// On validation error, zero bytes are written to `writer`.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        self.validate()?;
        self.write_raw(writer)
    }

    /// Validates `self` upfront and writes its binary representation to `writer` with explicit options.
    ///
    /// On validation error, zero bytes are written to `writer`.
    fn write_with_options<W: WriteBytes + BinSeek>(&self, writer: &mut W, options: &WriteOptions) -> Result<()> {
        self.validate()?;
        self.write_raw_with_options(writer, options)
    }

    /// Convenience method to validate and serialize `self` into a new `Vec<u8>` using default options.
    fn to_bytes(&self) -> Result<Vec<u8>> {
        self.to_bytes_with_options(&WriteOptions::default())
    }

    /// Convenience method to validate and serialize `self` into a new `Vec<u8>` with explicit options.
    fn to_bytes_with_options(&self, options: &WriteOptions) -> Result<Vec<u8>> {
        let mut buffer = BinMemoryBuffer::new();
        self.write_with_options(&mut buffer, options)?;
        Ok(buffer.as_slice().to_vec())
    }
}

/// Blanket implementation of `Writable` for slices of `Writable` elements.
impl<T: Writable> Writable for [T] {
    fn write_raw_with_options<W: WriteBytes + BinSeek>(&self, writer: &mut W, options: &WriteOptions) -> Result<()> {
        for item in self {
            item.write_raw_with_options(writer, options)?;
        }
        Ok(())
    }

    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        for item in self {
            item.write_raw(writer)?;
        }
        Ok(())
    }
}

/// Blanket implementation of `Writable` for `Vec<T>` where `T: Writable`.
impl<T: Writable> Writable for Vec<T> {
    fn write_raw_with_options<W: WriteBytes + BinSeek>(&self, writer: &mut W, options: &WriteOptions) -> Result<()> {
        self.as_slice().write_raw_with_options(writer, options)
    }

    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
        self.as_slice().write_raw(writer)
    }
}

/// A trait for types that can be validated and serialized to a binary stream using additional parameters.
pub trait WritableWithParams: Validatable {
    /// The type of parameters required for serialization.
    type P;

    /// Writes the binary representation of `self` with parameters and options without re-running validation.
    fn write_raw_with_options<W: WriteBytes + BinSeek>(
        &self,
        writer: &mut W,
        params: &Self::P,
        _options: &WriteOptions,
    ) -> Result<()>;

    /// Validates `self` upfront and writes its binary representation using `params` and default options.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W, params: &Self::P) -> Result<()> {
        self.write_with_options(writer, params, &WriteOptions::default())
    }

    /// Validates `self` upfront and writes its binary representation using `params` and explicit options.
    fn write_with_options<W: WriteBytes + BinSeek>(
        &self,
        writer: &mut W,
        params: &Self::P,
        options: &WriteOptions,
    ) -> Result<()> {
        self.validate()?;
        self.write_raw_with_options(writer, params, options)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::{BigEndian, BinSeek, WriteBytes};

    use super::*;

    struct SampleItem {
        id: u8,
        val: u16,
    }

    impl Validatable for SampleItem {
        fn validate(&self) -> Result<()> {
            if self.id == 0xFF {
                return Err(crate::Error::ValidationError("Invalid id".into()));
            }
            Ok(())
        }
    }

    impl Writable for SampleItem {
        fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> Result<()> {
            writer.write_u8(self.id)?;
            writer.write_u16::<BigEndian>(self.val)?;
            Ok(())
        }
    }

    #[test]
    fn test_writable_to_bytes() {
        let item = SampleItem { id: 0x01, val: 0x0203 };
        let bytes = item.to_bytes().unwrap();
        assert_eq!(bytes, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_blanket_writable_slice_and_vec() {
        let items = vec![SampleItem { id: 0x01, val: 0x0203 }, SampleItem { id: 0x04, val: 0x0506 }];
        let bytes = items.to_bytes().unwrap();
        assert_eq!(bytes, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        // If the second item is invalid, writing the Vec emits 0 bytes (atomic upfront validation)
        let invalid_items = vec![SampleItem { id: 0x01, val: 0x0203 }, SampleItem { id: 0xFF, val: 0x0506 }];
        let mut buffer = BinMemoryBuffer::new();
        assert!(invalid_items.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0);
    }
}
