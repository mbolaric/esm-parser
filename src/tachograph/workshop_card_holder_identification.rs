use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, HolderName, Name},
};

#[derive(Debug, Serialize)]
pub struct WorkshopCardHolderIdentification {
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub card_holder_name: HolderName,
    pub card_holder_preferred_language: String,
}

impl Readable<WorkshopCardHolderIdentification> for WorkshopCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<WorkshopCardHolderIdentification> {
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let card_holder_name = HolderName::read(reader)?;
        let card_holder_preferred_language = bytes_to_ia5_fix_string(&reader.read_into_vec(2)?)?;

        Ok(Self { workshop_name, workshop_address, card_holder_name, card_holder_preferred_language })
    }
}
