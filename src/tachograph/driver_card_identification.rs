use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, DriverCardHolderIdentification, EquipmentType},
};

#[derive(Debug, Serialize)]
pub struct DriverCardIdentification {
    pub card_identification: CardIdentification,
    pub driver_card_holder_identification: DriverCardHolderIdentification,
}

impl Readable<DriverCardIdentification> for DriverCardIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<DriverCardIdentification> {
        let card_identification_params = CardNumberParams::new(EquipmentType::DriverCard);
        let card_identification = CardIdentification::read(reader, &card_identification_params)?;
        let driver_card_holder_identification = DriverCardHolderIdentification::read(reader)?;
        Ok(Self { card_identification, driver_card_holder_identification })
    }
}
