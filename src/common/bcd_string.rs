use crate::{Error, Result};

#[derive(Debug)]
pub struct BCDString {}

impl BCDString {
    pub fn decode(bcd: &[u8]) -> Result<String> {
        let mut result = String::new();
        for byte in bcd {
            if (byte >> 4) & 0x0F > 9 || (byte & 0x0F) > 9 {
                return Err(Error::InvalidDataParse("Invalid BCDString input data.".to_owned()));
            }
            result.push(char::from(b'0' + ((byte >> 4) & 0x0F)));
            result.push(char::from(b'0' + (byte & 0x0F)));
        }
        Ok(result)
    }
}
