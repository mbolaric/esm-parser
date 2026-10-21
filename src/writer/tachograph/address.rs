use binary_data::{BinSeek, WriteBytes};

use crate::tacho::Address;
use crate::writer::{Validatable, Writable, validate_code_page_string, write_code_page_string};

impl Validatable for Address {
    /// Validates the address against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 35 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ Address::ADDRESS_LENGTH as usize }>("Address", &self.name, &self.code_page)
    }
}

impl Writable for Address {
    /// Writes an `Address` to a binary stream, conforming to the DDD file format.
    fn write_raw<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ Address::ADDRESS_LENGTH as usize }, _>(writer, "Address", &self.code_page, &self.name)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

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
