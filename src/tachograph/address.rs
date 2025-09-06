use crate::{CodePage, Readable, bytes_to_string};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Address {
    #[serde(rename = "codePage")]
    pub code_page: CodePage,
    pub name: String,
}

impl Readable<Address> for Address {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Address> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(35)?, &code_page).trim().to_string();
        Ok(Self { code_page, name })
    }
}
