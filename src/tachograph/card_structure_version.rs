use crate::Readable;

#[derive(Debug)]
pub struct CardStructureVersion {
    pub structure_version: u8,
    pub data_element_use_version: u8,
}

impl Readable<CardStructureVersion> for CardStructureVersion {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CardStructureVersion> {
        let structure_version = reader.read_u8()?;
        let data_element_use_version = reader.read_u8()?;
        Ok(Self { structure_version, data_element_use_version })
    }
}
