use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, ReadableWithParams, Result,
    tacho::{CardNumber, Name, NationNumericCode, TimeReal},
    tachograph::card_number::CardNumberParams,
};

#[derive(Debug)]
pub struct CardIdentification {}

impl ReadableWithParams<CardIdentification> for CardIdentification {
    type P = CardNumberParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardIdentification> {
        let card_issuing_member_state: NationNumericCode = reader.read_u8()?.into();
        let card_number = CardNumber::read(reader, &params)?;
        let card_issuing_authority_name = Name::read(reader)?;
        let card_issue_date = TimeReal::read(reader)?;
        let card_validity_begin = TimeReal::read(reader)?;
        let card_expiry_date = TimeReal::read(reader)?;

        Ok(Self {})
    }
}
