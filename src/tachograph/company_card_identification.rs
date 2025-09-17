use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, CompanyCardHolderIdentification, EquipmentType},
};

#[derive(Debug, Serialize)]
pub struct CompanyCardIdentification {
    pub card_identification: CardIdentification,
    pub company_card_holder_identification: CompanyCardHolderIdentification,
}

impl Readable<CompanyCardIdentification> for CompanyCardIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyCardIdentification> {
        let card_identification_params = CardNumberParams::new(EquipmentType::ControlCard);
        let card_identification = CardIdentification::read(reader, &card_identification_params)?;
        let company_card_holder_identification = CompanyCardHolderIdentification::read(reader)?;
        Ok(Self { card_identification, company_card_holder_identification })
    }
}
