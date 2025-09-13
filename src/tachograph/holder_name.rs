use crate::{Readable, tacho::Name};

/// This is the name and first name(s) of the holder of the Card.
#[derive(Debug)]
pub struct HolderName {
    pub surname: Name,
    pub first_name: Name,
}

impl Readable<HolderName> for HolderName {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<HolderName> {
        let surname = Name::read(reader)?;
        let first_name = Name::read(reader)?;
        Ok(Self { surname, first_name })
    }
}
