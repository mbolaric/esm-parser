use serde::Serialize;

use crate::{CodePage, Readable, Validatable, Writable, bytes_to_string, validate_code_page_string, write_code_page_string};

const NAME_LENGTH: u32 = 35;

/// A Name.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(type = "string"))]
pub struct Name {
    /// Specifies a character set.
    pub code_page: CodePage,
    /// This is a name encoded using the specified character set.
    pub name: String,
}

impl Validatable for Name {
    /// Validates the name against tachograph specifications:
    /// - Code page must be valid and supported.
    /// - All characters must be representable without loss in the specified code page.
    /// - Encoded byte length must not exceed 35 bytes.
    fn validate(&self) -> crate::Result<()> {
        validate_code_page_string::<{ NAME_LENGTH as usize }>("Name", &self.name, &self.code_page)
    }
}

impl Readable for Name {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Name> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(NAME_LENGTH)?, &code_page).trim().to_string();
        Ok(Self { code_page, name })
    }
}

impl Writable for Name {
    /// Writes the 36-byte binary representation of the name (1 byte code page + 35 bytes name padded with ASCII space).
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_code_page_string::<{ NAME_LENGTH as usize }, _>(writer, "Name", &self.code_page, &self.name)
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

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
