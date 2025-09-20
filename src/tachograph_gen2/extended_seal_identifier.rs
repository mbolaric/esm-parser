use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result};

/// The extended seal identifier uniquely identifies a seal (Annex IC requirement 401).
#[derive(Debug, Serialize)]
pub struct ExtendedSealIdentifier {
    /// see database registration to be managed by the European Commission (see https://dtc.jrc.ec.europa.eu).
    #[serde(rename = "manufacturerCode")]
    pub manufacturer_code: Vec<u8>,
    #[serde(rename = "sealIdentifier")]
    pub seal_identifier: Vec<u8>,
}

impl Readable<ExtendedSealIdentifier> for ExtendedSealIdentifier {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<ExtendedSealIdentifier> {
        let manufacturer_code = reader.read_into_vec(2)?;
        let seal_identifier = reader.read_into_vec(8)?;
        Ok(Self { manufacturer_code, seal_identifier })
    }
}
