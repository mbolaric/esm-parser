use serde::Serialize;

use crate::Readable;

/// Code indicating the version of the implemented structure in a tachograph card.
#[derive(Debug, Clone, Serialize)]
pub struct CardStructureVersion {
    #[serde(rename = "structureVersion")]
    pub structure_version: u8,
    #[serde(rename = "dataElementUseVersion")]
    pub data_element_use_version: u8,
}

impl Readable<CardStructureVersion> for CardStructureVersion {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardStructureVersion> {
        let structure_version = reader.read_u8()?;
        let data_element_use_version = reader.read_u8()?;
        Ok(Self { structure_version, data_element_use_version })
    }
}
