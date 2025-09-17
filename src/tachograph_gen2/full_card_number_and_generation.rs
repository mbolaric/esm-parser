use serde::Serialize;

use crate::{Readable, tacho::FullCardNumber};

#[derive(Debug, Serialize)]
pub struct FullCardNumberAndGeneration {
    pub full_card_number: FullCardNumber,
    pub generation: u8,
}

impl Readable<FullCardNumberAndGeneration> for FullCardNumberAndGeneration {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<FullCardNumberAndGeneration> {
        let full_card_number = FullCardNumber::read(reader)?;
        let generation = reader.read_u8()?;
        Ok(Self { full_card_number, generation })
    }
}
