use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, ControlCardHolderIdentification, EquipmentType},
};

/// Information, stored in a card, related to the identification of the card
/// (Annex 1C requirements 255, 280, 310, 333, 359, 365, 371, and 377).
#[derive(Debug, Serialize)]
pub struct ControlCardIdentification {
    #[serde(rename = "cardIdentification")]
    pub card_identification: CardIdentification,
    #[serde(rename = "companyCardHolderIdentification")]
    pub control_card_holder_identification: ControlCardHolderIdentification,
}

impl Readable<ControlCardIdentification> for ControlCardIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ControlCardIdentification> {
        let card_identification_params = CardNumberParams::new(EquipmentType::ControlCard);
        let card_identification = CardIdentification::read(reader, &card_identification_params)?;
        let control_card_holder_identification = ControlCardHolderIdentification::read(reader)?;
        Ok(Self { card_identification, control_card_holder_identification })
    }
}
