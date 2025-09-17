use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::FullCardNumberAndGeneration,
    tacho::{CardNumber, CardNumberParams, CardStructureVersion, EquipmentType, ExtendedSerialNumber},
};

#[derive(Debug, Serialize)]
pub struct VuCardRecord {
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub card_extended_serial_number: ExtendedSerialNumber,
    pub card_structure_version: CardStructureVersion,
    pub card_number: CardNumber,
}

impl Readable<VuCardRecord> for VuCardRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuCardRecord> {
        let full_card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let card_extended_serial_number = ExtendedSerialNumber::read(reader)?;
        let card_structure_version = CardStructureVersion::read(reader)?;
        let params = CardNumberParams::new(EquipmentType::DriverCard);
        let card_number = CardNumber::read(reader, &params)?;
        Ok(Self { full_card_number_and_generation, card_extended_serial_number, card_structure_version, card_number })
    }
}
