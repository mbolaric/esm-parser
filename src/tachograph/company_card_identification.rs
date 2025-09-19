use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{CardIdentification, CardNumberParams, CompanyCardHolderIdentification, EquipmentType},
};

/// Information, stored in a card, related to the identification of the card
/// (Annex 1C requirements 255, 280, 310, 333, 359, 365, 371, and 377).
#[derive(Debug, Serialize)]
pub struct CompanyCardIdentification {
    #[serde(rename = "cardIdentification")]
    pub card_identification: CardIdentification,
    #[serde(rename = "companyCardHolderIdentification")]
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
