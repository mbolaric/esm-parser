use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::FullCardNumberAndGeneration, helpers::u8_to_bool};

#[derive(Debug, Serialize)]
pub struct VuItsConsentRecord {
    pub card_number_and_generation: FullCardNumberAndGeneration,
    pub consent: bool,
}

impl Readable<VuItsConsentRecord> for VuItsConsentRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuItsConsentRecord> {
        let card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let consent: bool = u8_to_bool(reader.read_u8()?).is_err_and(|_| false);
        Ok(Self { card_number_and_generation, consent })
    }
}
