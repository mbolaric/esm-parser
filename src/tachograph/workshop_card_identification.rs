use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, EquipmentType, WorkshopCardHolderIdentification},
};

/// Information, stored in a card, related to the identification of the card
/// (Annex 1C requirements 255, 280, 310, 333, 359, 365, 371, and 377).
#[derive(Debug, Serialize)]
pub struct WorkshopCardIdentification {
    #[serde(rename = "cardIdentification")]
    pub card_identification: CardIdentification,
    #[serde(rename = "workshopCardHolderIdentification")]
    pub workshop_card_holder_identification: WorkshopCardHolderIdentification,
}

impl Readable<WorkshopCardIdentification> for WorkshopCardIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<WorkshopCardIdentification> {
        let card_identification_params = CardNumberParams::new(EquipmentType::WorkshopCard);
        let card_identification = CardIdentification::read(reader, &card_identification_params)?;
        let workshop_card_holder_identification = WorkshopCardHolderIdentification::read(reader)?;
        Ok(Self { card_identification, workshop_card_holder_identification })
    }
}
