use binary_data::{BinSeek, ReadBytes};

use crate::{
    Error, Readable, ReadableWithParams, Result,
    tacho::{
        CompanyCardIdentification, ControlCardIdentification, DriverCardIdentification, EquipmentType, WorkshopCardIdentification,
    },
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
    CompanyCard(Box<CompanyCardIdentification>),
    DriverCard(Box<DriverCardIdentification>),
    ControlCard(Box<ControlCardIdentification>),
    WorkshopCard(Box<WorkshopCardIdentification>),
}

impl ReadableWithParams<Identification> for Identification {
    type P = IdentificationParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Identification> {
        match params.equipment_type {
            EquipmentType::CompanyCard => Ok(Identification::CompanyCard(Box::new(CompanyCardIdentification::read(reader)?))),
            EquipmentType::DriverCard => Ok(Identification::DriverCard(Box::new(DriverCardIdentification::read(reader)?))),
            EquipmentType::ControlCard => Ok(Identification::ControlCard(Box::new(ControlCardIdentification::read(reader)?))),
            EquipmentType::WorkshopCard => Ok(Identification::WorkshopCard(Box::new(WorkshopCardIdentification::read(reader)?))),
            _ => Err(Error::UnknownCardTypeDecoding),
        }
    }
}
