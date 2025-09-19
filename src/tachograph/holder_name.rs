use serde::Serialize;

use crate::{Readable, tacho::Name};

/// This is the name and first name(s) of the holder of the Card.
#[derive(Debug, Serialize)]
pub struct HolderName {
    #[serde(rename = "holderSurname")]
    pub holder_surname: Name,
    #[serde(rename = "holderFirstNames")]
    pub holder_first_names: Name,
}

impl Readable<HolderName> for HolderName {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<HolderName> {
        let holder_surname = Name::read(reader)?;
        let holder_first_names = Name::read(reader)?;
        Ok(Self { holder_surname, holder_first_names })
    }
}
