use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, Name},
};

#[derive(Debug)]
pub struct CompanyCardHolderIdentification {
    pub company_name: Name,
    pub company_address: Address,
    pub card_holder_preferred_language: String,
}

impl Readable<CompanyCardHolderIdentification> for CompanyCardHolderIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CompanyCardHolderIdentification> {
        let company_name = Name::read(reader)?;
        let company_address = Address::read(reader)?;
        let card_holder_preferred_language = bytes_to_ia5_fix_string(&reader.read_into_vec(2)?)?;

        Ok(Self { company_name, company_address, card_holder_preferred_language })
    }
}
