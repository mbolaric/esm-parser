use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, HolderName, Name},
};

const CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH: u32 = 2;

/// Information, stored in a workshop card, related to the identification of
/// the cardholder (Annex 1C requirement 311 and 334).
#[derive(Debug, Serialize)]
pub struct WorkshopCardHolderIdentification {
    #[serde(rename = "workshopName")]
    pub workshop_name: Name,
    #[serde(rename = "calibrationTotalNumber")]
    pub workshop_address: Address,
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: HolderName,
    #[serde(rename = "cardHolderPreferredLanguage")]
    pub card_holder_preferred_language: String,
}

impl Readable<WorkshopCardHolderIdentification> for WorkshopCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<WorkshopCardHolderIdentification> {
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_preferred_language =
            bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH)?)?;

        Ok(Self { workshop_name, workshop_address, card_holder_name, card_holder_preferred_language })
    }
}
