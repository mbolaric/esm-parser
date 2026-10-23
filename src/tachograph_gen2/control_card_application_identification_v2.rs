use binary_data::BigEndian;
use serde::Serialize;

use crate::{Error, Readable, Result};

/// Information, stored in a control card related to the identification of the
/// application of the card (Annex IC requirement 363a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2ControlCardApplicationIdentificationV2"))]
pub struct ControlCardApplicationIdentificationV2 {
    #[serde(rename = "lengthOfFollowingData")]
    pub length_of_following_data: u16,
    #[serde(rename = "vuConfigurationLengthRange")]
    pub vu_configuration_length_range: u16,
}

impl Readable for ControlCardApplicationIdentificationV2 {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> Result<ControlCardApplicationIdentificationV2> {
        let length_of_following_data = reader.read_u16::<BigEndian>()?;
        if length_of_following_data != 2 {
            return Err(Error::InvalidDataParse(format!(
                "Unsupported control ApplicationIdentificationV2 lengthOfFollowingData: {length_of_following_data}, expected 2"
            )));
        }
        let vu_configuration_length_range = reader.read_u16::<BigEndian>()?;

        Ok(Self { length_of_following_data, vu_configuration_length_range })
    }
}
