use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::ExtendedSealIdentifier, tacho::EquipmentType};

#[derive(Debug, Serialize)]
pub struct SealRecord {
    pub equipment_type: EquipmentType,
    pub extended_seal_identitfier: ExtendedSealIdentifier,
}

impl Readable<SealRecord> for SealRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealRecord> {
        let equipment_type: EquipmentType = reader.read_u8()?.into();
        let extended_seal_identitfier = ExtendedSealIdentifier::read(reader)?;

        Ok(Self { equipment_type, extended_seal_identitfier })
    }
}
