use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::FullCardNumberAndGeneration;
use crate::{Readable, Result};

/// Information stored in a vehicle unit, related to the consent of a driver to
/// use Intelligent Transport Systems.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuItsConsentRecord"))]
pub struct VuItsConsentRecord {
    #[serde(rename = "cardNumberAndGen")]
    pub card_number_and_generation: FullCardNumberAndGeneration,
    pub consent: bool,
}

impl Readable<VuItsConsentRecord> for VuItsConsentRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuItsConsentRecord> {
        let card_number_and_generation = FullCardNumberAndGeneration::read(reader)?;
        let consent: bool = reader.read_u8()? == 1;
        Ok(Self { card_number_and_generation, consent })
    }
}
