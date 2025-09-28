use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, HolderName, Name},
};

const CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH: u32 = 2;

/// Information, stored in a control card, related to the identification of the
/// cardholder (Annex 1C requirement 360 and 366).
#[derive(Debug, Serialize)]
pub struct ControlCardHolderIdentification {
    #[serde(rename = "controlBodyName")]
    pub control_body_name: Name,
    #[serde(rename = "controlBodyAddress")]
    pub control_body_address: Address,
    #[serde(rename = "cardHolderName")]
    pub card_holder_name: HolderName,
    #[serde(rename = "cardHolderPreferredLanguage")]
    pub card_holder_preferred_language: String,
}

impl Readable<ControlCardHolderIdentification> for ControlCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ControlCardHolderIdentification> {
        let control_body_name = Name::read(reader)?;
        let control_body_address = Address::read(reader)?;
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_preferred_language =
            bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH)?)?;

        Ok(Self { control_body_name, control_body_address, card_holder_name, card_holder_preferred_language })
    }
}
