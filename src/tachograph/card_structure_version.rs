use serde::Serialize;

use crate::{Readable, Validatable, Writable};

/// Code indicating the version of the implemented structure in a tachograph card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardStructureVersion {
    #[serde(rename = "structureVersion")]
    pub structure_version: u8,
    #[serde(rename = "dataElementUseVersion")]
    pub data_element_use_version: u8,
}

impl Validatable for CardStructureVersion {
    fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}

impl Readable for CardStructureVersion {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardStructureVersion> {
        let structure_version = reader.read_u8()?;
        let data_element_use_version = reader.read_u8()?;
        Ok(Self { structure_version, data_element_use_version })
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

    use super::*;

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
