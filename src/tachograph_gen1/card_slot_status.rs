use crate::{tacho::CardSlotStatusCode, Readable};

#[derive(Debug)]
pub struct CardSlotStatus {
    pub data: u8,
    pub driver_slot: CardSlotStatusCode,
    pub co_driver_slot: CardSlotStatusCode,
}

impl Readable<CardSlotStatus> for CardSlotStatus {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CardSlotStatus> {
        let data = reader.read_u8()?;
        let co_driver_slot: CardSlotStatusCode = ((data >> 4) & 0xF).into();
        let driver_slot: CardSlotStatusCode = (data & 0xF).into();
        Ok(Self {
            data,
            driver_slot,
            co_driver_slot,
        })
    }
}
