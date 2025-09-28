use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Datef, HolderName},
};

const CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH: u32 = 2;

/// Information, stored in a driver card, related to the identification of the
/// cardholder (Annex 1C requirement 256 and 281).
#[derive(Debug, Serialize)]
pub struct DriverCardHolderIdentification {
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: HolderName,
    #[serde(rename = "cardHolderBirthDate")]
    pub card_holder_birth_date: Datef,
    #[serde(rename = "cardHolderPreferredLanguage")]
    pub card_holder_preferred_language: String,
}

impl Readable<DriverCardHolderIdentification> for DriverCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<DriverCardHolderIdentification> {
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_birth_date = Datef::read(reader)?;
        let card_holder_preferred_language =
            bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH)?)?;

        Ok(Self { card_holder_name, card_holder_birth_date, card_holder_preferred_language })
    }
}
