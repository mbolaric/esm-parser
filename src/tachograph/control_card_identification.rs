use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, ControlCardHolderIdentification, EquipmentType},
};

#[derive(Debug, Serialize)]
pub struct ControlCardIdentification {
    pub card_identification: CardIdentification,
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
