use crate::{helpers::vec_u8_to_string, Readable};

#[derive(Debug)]
pub struct Address {
    pub code_page: u8,
    pub name: String,
}

impl Readable<Address> for Address {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<Address> {
        let code_page = reader.read_u8()?;
        let name = vec_u8_to_string(reader.read_into_vec(35)?)?
            .trim()
            .to_string();

        Ok(Self { code_page, name })
    }
}
