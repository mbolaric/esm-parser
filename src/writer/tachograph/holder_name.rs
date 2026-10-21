use crate::tacho::HolderName;
use crate::writer::{Validatable, Writable};

impl Validatable for HolderName {
    /// Validates both the surname and first name(s) according to tachograph specifications.
    fn validate(&self) -> crate::Result<()> {
        self.holder_surname.validate()?;
        self.holder_first_names.validate()?;
        Ok(())
    }
}

impl Writable for HolderName {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        self.holder_surname.write_raw(writer)?;
        self.holder_first_names.write_raw(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};
    use crate::{CodePage, Readable};

    #[test]
    fn test_holder_name_roundtrip() {
        let holder = HolderName {
            holder_surname: Name { code_page: CodePage::IsoIec8859_1, name: "Mustermann".to_string() },
            holder_first_names: Name { code_page: CodePage::IsoIec8859_1, name: "Max".to_string() },
        };

        let bytes = holder.to_bytes().unwrap();
        assert_eq!(bytes.len(), 72); // 36 + 36

        let mut reader = BinMemoryBuffer::from(bytes);
        let parsed = HolderName::read(&mut reader).unwrap();
        assert_eq!(parsed, holder);
    }

    #[test]
    fn test_holder_name_validation_and_zero_byte_emission() {
        // Valid holder name
        let valid_holder = HolderName {
            holder_surname: Name { code_page: CodePage::IsoIec8859_1, name: "Mustermann".to_string() },
            holder_first_names: Name { code_page: CodePage::IsoIec8859_1, name: "Max".to_string() },
        };
        assert!(valid_holder.validate().is_ok());

        // Overlong surname: 36 characters
        let invalid_holder = HolderName {
            holder_surname: Name { code_page: CodePage::IsoIec8859_1, name: "A".repeat(36) },
            holder_first_names: Name { code_page: CodePage::IsoIec8859_1, name: "Max".to_string() },
        };
        assert!(invalid_holder.validate().is_err());

        // Verify that zero bytes are emitted on failure
        let mut buffer = BinMemoryBuffer::new();
        assert!(invalid_holder.write(&mut buffer).is_err());
        assert_eq!(buffer.as_slice().len(), 0);
    }
}
