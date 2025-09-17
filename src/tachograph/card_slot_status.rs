use serde::Serialize;

use crate::{Readable, tacho::CardSlotStatusType};

#[derive(Debug, Serialize)]
pub struct CardSlotStatus {
    pub data: u8,
    pub driver_slot: CardSlotStatusType,
    pub co_driver_slot: CardSlotStatusType,
}

impl Readable<CardSlotStatus> for CardSlotStatus {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardSlotStatus> {
        let data = reader.read_u8()?;
        let co_driver_slot: CardSlotStatusType = ((data >> 4) & 0xF).into();
        let driver_slot: CardSlotStatusType = (data & 0xF).into();
        Ok(Self { data, driver_slot, co_driver_slot })
    }
}
