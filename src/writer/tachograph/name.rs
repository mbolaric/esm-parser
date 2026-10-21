use crate::tacho::Name;
use crate::writer::{Validatable, Writable, validate_code_page_string, write_code_page_string};

impl Validatable for Name {
    /// Validates the name against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 35 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ Name::NAME_LENGTH as usize }>("Name", &self.name, &self.code_page)
    }
}

impl Writable for Name {
    /// Writes the 36-byte binary representation of the name (1 byte code page + 35 bytes name padded with ASCII space).
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ Name::NAME_LENGTH as usize }, _>(writer, "Name", &self.code_page, &self.name)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_name_write_and_read_roundtrip() {
        let name = Name { code_page: CodePage::IsoIec8859_1, name: "Musterfrau".to_string() };
        let bytes = name.to_bytes().unwrap();
        assert_eq!(bytes.len(), 36); // 1 byte code_page + 35 bytes name

        let mut reader = BinMemoryBuffer::from(bytes);
        let parsed = Name::read(&mut reader).unwrap();
        assert_eq!(parsed, name);
    }

    #[test]
    fn test_name_validation_and_zero_byte_emission() {
        // Exactly 35 bytes name (valid)
        let exact_35 = Name { code_page: CodePage::IsoIec8859_1, name: "A".repeat(35) };
        assert!(exact_35.validate().is_ok());
        let bytes = exact_35.to_bytes().unwrap();
        assert_eq!(bytes.len(), 36);

        // Overlong name: 36 bytes (must fail validation)
        let overlong = Name { code_page: CodePage::IsoIec8859_1, name: "A".repeat(36) };
        assert!(overlong.validate().is_err());
        let mut buffer = BinMemoryBuffer::new();
        assert!(overlong.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0); // 0 bytes emitted on error

        // Invalid code page (must fail validation)
        let invalid_cp = Name { code_page: CodePage::Invalid, name: "Alice".into() };
        assert!(invalid_cp.validate().is_err());
        let mut buffer2 = BinMemoryBuffer::new();
        assert!(invalid_cp.write(&mut buffer2).is_err());
        assert_eq!(buffer2.as_slice().len(), 0);

        // Unencodable character for ISO-8859-1 (must fail validation)
        let unencodable = Name { code_page: CodePage::IsoIec8859_1, name: "Alice €".into() };
        assert!(unencodable.validate().is_err());
        let mut buffer3 = BinMemoryBuffer::new();
        assert!(unencodable.write(&mut buffer3).is_err());
        assert_eq!(buffer3.as_slice().len(), 0);
    }
}
