use binary_data::{BinSeek, WriteBytes};
use serde::Serialize;

use crate::{CodePage, Readable, Validatable, Writable, bytes_to_string, validate_code_page_string, write_code_page_string};

const ADDRESS_LENGTH: u32 = 35;

/// Represents a postal address, typically used for company or workshop locations in a DDD file.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(type = "string"))]
pub struct Address {
    /// The code page used for encoding the address string.
    pub code_page: CodePage,
    /// The address, with a fixed length of 35 bytes.
    pub name: String,
}

impl Validatable for Address {
    /// Validates the address against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 35 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ ADDRESS_LENGTH as usize }>("Address", &self.name, &self.code_page)
    }
}

impl Readable for Address {
    /// Reads an `Address` from a binary stream, as specified by the DDD file format.
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Address> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(ADDRESS_LENGTH)?, &code_page).trim().to_string();
        Ok(Self { code_page, name })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl Writable for Address {
    /// Writes an `Address` to a binary stream, conforming to the DDD file format.
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ ADDRESS_LENGTH as usize }, _>(writer, "Address", &self.code_page, &self.name)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn test_address_roundtrip() {
        let addr = Address { code_page: CodePage::IsoIec8859_1, name: "Musterstraße 1, Berlin".to_string() };
        let bytes = addr.to_bytes().unwrap();
        assert_eq!(bytes.len(), 36);

        let mut reader = BinMemoryBuffer::from(bytes);
        let parsed = Address::read(&mut reader).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_address_validation_and_zero_byte_emission() {
        let overlong = Address { code_page: CodePage::IsoIec8859_1, name: "A".repeat(36) };
        assert!(overlong.validate().is_err());
        let mut buffer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0);

        let invalid_cp = Address { code_page: CodePage::Invalid, name: "Street 1".into() };
        assert!(invalid_cp.validate().is_err());
        let mut buffer2 = BinMemoryBuffer::new();
        assert!(invalid_cp.write(&mut buffer2).is_err());
        assert_eq!(buffer2.as_slice().len(), 0);

        let unencodable = Address { code_page: CodePage::IsoIec8859_1, name: "Street €".into() };
        assert!(unencodable.validate().is_err());
        let mut buffer3 = BinMemoryBuffer::new();
        assert!(unencodable.write(&mut buffer3).is_err());
        assert_eq!(buffer3.as_slice().len(), 0);
    }
}
