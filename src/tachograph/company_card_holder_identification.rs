use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, Name},
};

const CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH: u32 = 2;

/// Information, stored in a company card, related to the cardholder identification
/// (Annex 1C requirement 372 and 378).
#[derive(Debug, Serialize)]
pub struct CompanyCardHolderIdentification {
    #[serde(rename = "companyName")]
    pub company_name: Name,
    #[serde(rename = "companyAddress")]
    pub company_address: Address,
    #[serde(rename = "cardHolderPreferredLanguage")]
    pub card_holder_preferred_language: String,
}

impl Readable<CompanyCardHolderIdentification> for CompanyCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyCardHolderIdentification> {
        let company_name = Name::read(reader)?;
        let company_address = Address::read(reader)?;
        let card_holder_preferred_language =
            bytes_to_ia5_fix_string(&reader.read_into_vec(CARD_HOLDER_PREFERRED_LANGUAGE_LENGTH)?)?;

        Ok(Self { company_name, company_address, card_holder_preferred_language })
    }
}
