use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    tacho::{CardNumber, Name, NationNumeric, TimeReal},
    tachograph::card_number::CardNumberParams,
};

/// Information, stored in a card, related to the identification of the card
/// (Annex 1C requirements 255, 280, 310, 333, 359, 365, 371, and 377).
#[derive(Debug, Serialize)]
pub struct CardIdentification {
    #[serde(rename = "cardIssuingMemberState")]
    pub card_issuing_member_state: NationNumeric,
    #[serde(rename = "cardNumber")]
    pub card_number: CardNumber,
    #[serde(rename = "cardIssuingAuthorityName")]
    pub card_issuing_authority_name: Name,
    #[serde(rename = "cardIssueDate")]
    pub card_issue_date: TimeReal,
    #[serde(rename = "cardValidityBegin")]
    pub card_validity_begin: TimeReal,
    #[serde(rename = "cardExpiryDate")]
    pub card_expiry_date: TimeReal,
}

impl ReadableWithParams<CardIdentification> for CardIdentification {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardIdentification> {
        let card_issuing_member_state: NationNumeric = reader.read_u8()?.into();
        let card_number = CardNumber::read(reader, params)?;
        let card_issuing_authority_name = Name::read(reader)?;
        let card_issue_date = TimeReal::read(reader)?;
        let card_validity_begin = TimeReal::read(reader)?;
        let card_expiry_date = TimeReal::read(reader)?;

        Ok(Self {
            card_issuing_member_state,
            card_number,
            card_issuing_authority_name,
            card_issue_date,
            card_validity_begin,
            card_expiry_date,
        })
    }
}
