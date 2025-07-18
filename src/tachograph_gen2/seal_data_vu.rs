use binary_data::{BinSeek, ReadBytes};

use crate::{Readable, Result, gen2::ExtendedSealIdentifier, tacho::EquipmentType};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct SealDataVu {
    pub seal_records: Vec<SealRecord>,
}

impl Readable<SealDataVu> for SealDataVu {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SealDataVu> {
        let mut seal_records: Vec<SealRecord> = Vec::with_capacity(5);
        for _ in 0..5 {
            let record = SealRecord::read(reader)?;
            seal_records.push(record);
        }
        Ok(Self { seal_records })
    }
}
