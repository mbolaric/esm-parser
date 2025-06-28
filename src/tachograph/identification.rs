use binary_data::{BinSeek, ReadBytes};

use crate::{
    Error, Readable, ReadableWithParams, Result,
    tacho::{DriverCardIdentification, EquipmentType},
};

#[derive(Debug)]
pub struct IdentificationParams {
    pub equipment_type: EquipmentType,
}

impl IdentificationParams {
    pub fn new(equipment_type: EquipmentType) -> Self {
        Self { equipment_type }
    }
}

#[derive(Debug)]
pub enum Identification {
    CompanyCard,
    DriverCard(DriverCardIdentification),
    ControlCard,
    WorkshopCard,
}

impl ReadableWithParams<Identification> for Identification {
    type P = IdentificationParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Identification> {
        match params.equipment_type {
            EquipmentType::CompanyCard => Err(Error::NotImplemented),
            EquipmentType::DriverCard => Ok(Identification::DriverCard(DriverCardIdentification::read(reader)?)),
            EquipmentType::ControlCard => Err(Error::NotImplemented),
            EquipmentType::WorkshopCard => Err(Error::NotImplemented),
            _ => Err(Error::UnknownCardTypeDecoding),
        }
    }
}
