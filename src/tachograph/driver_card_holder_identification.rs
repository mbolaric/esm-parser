use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{HolderName, StringDate},
};

#[derive(Debug)]
pub struct DriverCardHolderIdentification {
    pub card_holder_name: HolderName,
    pub card_holder_birth_date: StringDate,
    pub card_holder_preferred_language: String,
}

impl Readable<DriverCardHolderIdentification> for DriverCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<DriverCardHolderIdentification> {
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_birth_date = StringDate::read(reader)?;
        let card_holder_preferred_language = bytes_to_ia5_fix_string(&reader.read_into_vec(2)?)?;

        Ok(Self { card_holder_name, card_holder_birth_date, card_holder_preferred_language })
    }
}
