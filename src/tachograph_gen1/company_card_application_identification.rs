use binary_data::BigEndian;

use crate::{
    Readable,
    tacho::{CardStructureVersion, EquipmentType},
};

#[derive(Debug, Clone)]
pub struct CompanyCardApplicationIdentification {
    pub type_of_tachograph_card_id: EquipmentType,
    pub card_structure_version: CardStructureVersion,
    pub no_of_company_activity_records: u32,
}

impl Readable<CompanyCardApplicationIdentification> for CompanyCardApplicationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CompanyCardApplicationIdentification> {
        let type_of_tachograph_card_id = reader.read_u8()?.into();
        let card_structure_version = CardStructureVersion::read(reader)?;
        let no_of_company_activity_records = reader.read_u16::<BigEndian>()? as u32;

        Ok(Self { type_of_tachograph_card_id, card_structure_version, no_of_company_activity_records })
    }
}
