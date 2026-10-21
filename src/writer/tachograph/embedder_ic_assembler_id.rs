use crate::tacho::EmbedderIcAssemblerId;
use crate::writer::{Validatable, Writable, validate_bcd, validate_fixed_bytes, validate_ia5_string, write_ia5_string};

impl Validatable for EmbedderIcAssemblerId {
    fn validate(&self) -> crate::Result<()> {
        validate_ia5_string::<{ EmbedderIcAssemblerId::COUNTRY_CODE_LENGTH as usize }>(
            "EmbedderIcAssemblerId.country_code",
            &self.country_code,
        )?;
        validate_bcd::<{ EmbedderIcAssemblerId::MODULE_EMBEDDER_LENGTH }>(
            "EmbedderIcAssemblerId.module_embedder",
            &self.module_embedder,
        )?;
        validate_fixed_bytes::<{ EmbedderIcAssemblerId::MANUFACTURER_INFORMATION_LENGTH as usize }>(
            "EmbedderIcAssemblerId.manufacturer_information",
            &self.manufacturer_information,
        )
    }
}

impl Writable for EmbedderIcAssemblerId {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        write_ia5_string::<{ EmbedderIcAssemblerId::COUNTRY_CODE_LENGTH as usize }, _>(
            writer,
            "EmbedderIcAssemblerId.country_code",
            &self.country_code,
        )?;
        let embedder_bytes = validate_bcd::<{ EmbedderIcAssemblerId::MODULE_EMBEDDER_LENGTH }>(
            "EmbedderIcAssemblerId.module_embedder",
            &self.module_embedder,
        )?;
        writer.write_all(&embedder_bytes)?;
        writer.write_all(&self.manufacturer_information)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use crate::Readable;
    use crate::tacho::*;
    use crate::writer::{Validatable, Writable};

    #[test]
    fn test_embedder_ic_assembler_id_roundtrip() {
        let data = vec![b'D', b'E', 0x12, 0xAA, 0xBB];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let id = EmbedderIcAssemblerId::read(&mut reader).unwrap();
        assert_eq!(id.country_code, "DE");
        assert_eq!(id.module_embedder, "12");
        assert_eq!(id.manufacturer_information, vec![0xAA, 0xBB]);

        let mut writer = BinMemoryBuffer::new();
        id.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }

    #[test]
    fn test_embedder_ic_assembler_id_validation_and_zero_byte_emission() {
        // Invalid BCD in module_embedder after valid country_code
        let invalid_bcd = EmbedderIcAssemblerId {
            country_code: "DE".into(),
            module_embedder: "AB".into(),
            manufacturer_information: vec![0x01, 0x02],
        };
        assert!(invalid_bcd.validate().is_err());
        let mut writer = BinMemoryBuffer::new();
        assert!(invalid_bcd.write(&mut writer).is_err());
        assert_eq!(writer.as_slice().len(), 0);

        // Wrong manufacturer_information length
        let invalid_man = EmbedderIcAssemblerId {
            country_code: "DE".into(),
            module_embedder: "12".into(),
            manufacturer_information: vec![0x01],
        };
        assert!(invalid_man.validate().is_err());
        let mut writer2 = BinMemoryBuffer::new();
        assert!(invalid_man.write(&mut writer2).is_err());
        assert_eq!(writer2.as_slice().len(), 0);
    }
}
