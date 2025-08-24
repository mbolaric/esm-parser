use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, EquipmentType, WorkshopCardHolderIdentification},
};

#[derive(Debug)]
pub struct WorkshopCardIdentification {
    pub card_identification: CardIdentification,
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
