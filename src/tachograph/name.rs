use serde::Serialize;

use crate::{CodePage, Readable, bytes_to_string};

const NAME_LENGTH: u32 = 35;

/// A Name.
#[derive(Debug)]
pub struct Name {
    /// Specifies a character set.
    pub code_page: CodePage,
    /// This is a name encoded using the specified character set.
    pub name: String,
}

impl Readable<Name> for Name {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Name> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(NAME_LENGTH)?, &code_page).trim().to_string();
        Ok(Self { code_page, name })
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}
