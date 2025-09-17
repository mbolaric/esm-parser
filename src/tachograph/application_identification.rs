use serde::Serialize;

use crate::{
    Readable,
    tacho::{CardStructureVersion, EquipmentType},
};

#[derive(Debug, Serialize)]
pub struct ApplicationIdentification {
    pub type_of_tachograph_card_id: EquipmentType,
    pub card_structure_version: CardStructureVersion,
}

impl Readable<ApplicationIdentification> for ApplicationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ApplicationIdentification> {
        let type_of_tachograph_card_id = reader.read_u8()?.into();
        let card_structure_version = CardStructureVersion::read(reader)?;
        Ok(Self { type_of_tachograph_card_id, card_structure_version })
    }
}
