use binary_data::BigEndian;
use serde::Serialize;

use crate::Readable;

/// Information, stored in a company card related to the identification of the
/// application of the card (Annex IC requirement 375a).
#[derive(Debug, Serialize)]
pub struct CompanyCardApplicationIdentificationV2 {
    #[serde(rename = "lengthOfFollowingData")]
    pub length_of_following_data: u16,
    #[serde(rename = "vuConfigurationLengthRange")]
    pub vu_configuration_length_range: u16,
}

impl Readable<CompanyCardApplicationIdentificationV2> for CompanyCardApplicationIdentificationV2 {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CompanyCardApplicationIdentificationV2> {
        let length_of_following_data = reader.read_u16::<BigEndian>()?;
        let vu_configuration_length_range = reader.read_u16::<BigEndian>()?;

        Ok(Self { length_of_following_data, vu_configuration_length_range })
    }
}
