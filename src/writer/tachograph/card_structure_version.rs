use crate::tacho::CardStructureVersion;
use crate::writer::{Validatable, Writable};

impl Validatable for CardStructureVersion {
    fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}

impl Writable for CardStructureVersion {
    fn write_raw<W: binary_data::WriteBytes + binary_data::BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.structure_version)?;
        writer.write_u8(self.data_element_use_version)?;
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
    fn test_card_structure_version_roundtrip() {
        let data = vec![0x01, 0x02];
        let mut reader = BinMemoryBuffer::from(data.clone());
        let version = CardStructureVersion::read(&mut reader).unwrap();
        assert_eq!(version.structure_version, 1);
        assert_eq!(version.data_element_use_version, 2);
        assert!(version.validate().is_ok());

        let mut writer = BinMemoryBuffer::new();
        version.write(&mut writer).unwrap();
        assert_eq!(writer.as_slice(), data.as_slice());
    }
}
