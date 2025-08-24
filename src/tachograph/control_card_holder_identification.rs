use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, HolderName, Name},
};

#[derive(Debug)]
pub struct ControlCardHolderIdentification {
    pub control_body_name: Name,
    pub control_body_address: Address,
    pub card_holder_name: HolderName,
    pub card_holder_preferred_language: String,
}

impl Readable<ControlCardHolderIdentification> for ControlCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ControlCardHolderIdentification> {
        let control_body_name = Name::read(reader)?;
        let control_body_address = Address::read(reader)?;
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_preferred_language = bytes_to_ia5_fix_string(&reader.read_into_vec(2)?)?;

        Ok(Self { control_body_name, control_body_address, card_holder_name, card_holder_preferred_language })
    }
}
