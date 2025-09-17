use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result};

#[derive(Debug, Serialize)]
pub struct ExtendedSealIdentifier {
    pub manufacturer_code: Vec<u8>,
    pub seal_identifier: Vec<u8>,
}

impl Readable<ExtendedSealIdentifier> for ExtendedSealIdentifier {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<ExtendedSealIdentifier> {
        let manufacturer_code = reader.read_into_vec(2)?;
        let seal_identifier = reader.read_into_vec(8)?;
        Ok(Self { manufacturer_code, seal_identifier })
    }
}
