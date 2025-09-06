use crate::{CodePage, Readable, Writable, bytes_to_string, string_to_bytes};
use binary_data::{BinSeek, WriteBytes};
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

impl Writable for Address {
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.code_page.clone() as u8)?;
        let mut name_bytes = string_to_bytes(&self.name, &self.code_page);
        name_bytes.resize(35, 0);
        writer.write_all(&name_bytes)?;
        Ok(())
    }
}
