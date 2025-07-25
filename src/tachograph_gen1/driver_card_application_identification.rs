use binary_data::BigEndian;

use crate::{
    Readable,
    tacho::{CardStructureVersion, EquipmentType},
};

#[derive(Debug, Clone)]
pub struct DriverCardApplicationIdentification {
    pub type_of_tachograph_card_id: EquipmentType,
    pub card_structure_version: CardStructureVersion,
    pub no_events_per_type: u8,
    pub no_faults_per_type: u8,
    pub card_activity_length_range: u32,
    pub no_of_card_vehicle_records: u32,
    pub no_of_card_place_records: u32,
}

impl Readable<DriverCardApplicationIdentification> for DriverCardApplicationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<DriverCardApplicationIdentification> {
        let type_of_tachograph_card_id = reader.read_u8()?.into();
        let card_structure_version = CardStructureVersion::read(reader)?;
        let no_events_per_type = reader.read_u8()?;
        let no_faults_per_type = reader.read_u8()?;
        let card_activity_length_range = reader.read_u16::<BigEndian>()? as u32;
        let no_of_card_vehicle_records = reader.read_u16::<BigEndian>()? as u32;
        let no_of_place_records = reader.read_u8()? as u32;

        Ok(Self {
            type_of_tachograph_card_id,
            card_structure_version,
            no_events_per_type,
            no_faults_per_type,
            card_activity_length_range,
            no_of_card_vehicle_records,
            no_of_card_place_records: no_of_place_records,
        })
    }
}
