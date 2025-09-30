use crate::{CodePage, Readable, Writable, bytes_to_string, string_to_bytes};
use binary_data::{BinSeek, WriteBytes};
use serde::Serialize;

const ADDRESS_LENGTH: u32 = 35;

/// Represents a postal address, typically used for company or workshop locations in a DDD file.
#[derive(Debug)]
pub struct Address {
    /// The code page used for encoding the address string.
    pub code_page: CodePage,
    /// The address, with a fixed length of 35 bytes.
    pub name: String,
}

impl Readable<Address> for Address {
    /// Reads an `Address` from a binary stream, as specified by the DDD file format.
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Address> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(ADDRESS_LENGTH)?, &code_page).trim().to_string();
        Ok(Self { code_page, name })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl Writable for Address {
    /// Writes an `Address` to a binary stream, conforming to the DDD file format.
    fn write<W: WriteBytes + BinSeek>(&self, writer: &mut W) -> crate::Result<()> {
        writer.write_u8(self.code_page.clone() as u8)?;
        let mut name_bytes = string_to_bytes(&self.name, &self.code_page);
        name_bytes.resize(ADDRESS_LENGTH as usize, 0);
        writer.write_all(&name_bytes)?;
        Ok(())
    }
}
