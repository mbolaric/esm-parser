use serde::Serialize;

use crate::{CodePage, Readable, bytes_to_string};

/// Represents a postal address, typically used for company or workshop locations in a DDD file.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(type = "string"))]
pub struct Address {
    /// The code page used for encoding the address string.
    pub code_page: CodePage,
    /// The address, with a fixed length of 35 bytes.
    pub name: String,
}

impl Address {
    pub(crate) const ADDRESS_LENGTH: u32 = 35;
}

impl Readable for Address {
    /// Reads an `Address` from a binary stream, as specified by the DDD file format.
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Address> {
        let code_page: CodePage = reader.read_u8()?.into();
        let name = bytes_to_string(&reader.read_into_vec(Address::ADDRESS_LENGTH)?, &code_page).trim().to_string();
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
