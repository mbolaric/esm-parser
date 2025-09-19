use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, DriverCardHolderIdentification, EquipmentType},
};

/// Information, stored in a card, related to the identification of the card
/// (Annex 1C requirements 255, 280, 310, 333, 359, 365, 371, and 377).
#[derive(Debug, Serialize)]
pub struct DriverCardIdentification {
    #[serde(rename = "cardIdentification")]
    pub card_identification: CardIdentification,
    #[serde(rename = "driverCardHolderIdentification")]
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
