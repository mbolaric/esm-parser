use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::ExtendedSealIdentifier, tacho::EquipmentType};

/// This data type stores information about a seal that is attached to a
/// component. This data type is related to Annex 1C requirement 337.
#[derive(Debug, Serialize)]
pub struct SealRecord {
    #[serde(rename = "equipmentType")]
    pub equipment_type: EquipmentType,
    #[serde(rename = "extendedSealIdentifier")]
    pub extended_seal_identitfier: ExtendedSealIdentifier,
}

impl Readable<SealRecord> for SealRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealRecord> {
        let equipment_type: EquipmentType = reader.read_u8()?.into();
        let extended_seal_identitfier = ExtendedSealIdentifier::read(reader)?;

        Ok(Self { equipment_type, extended_seal_identitfier })
    }
}
