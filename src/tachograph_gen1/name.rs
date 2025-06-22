use log::trace;

use crate::{helpers::vec_u8_to_string, Readable};

#[derive(Debug)]
pub struct Name {
    pub code_page: u8,
    pub name: String,
}

impl Readable<Name> for Name {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<Name> {
        // TODO: for now is not used
        let code_page = reader.read_u8()?;
        trace!("Name::read - Code Page: {:?}", code_page);
        let name = vec_u8_to_string(reader.read_into_vec(35)?)?
            .trim()
            .to_string();

        Ok(Self { code_page, name })
    }
}
