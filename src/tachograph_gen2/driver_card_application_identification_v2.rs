use binary_data::BigEndian;
use serde::Serialize;

use crate::Readable;

/// Information, stored in a driver card related to the identification of the
/// application of the card (Annex IC requirement 375a).
#[derive(Debug, Serialize)]
pub struct DriverCardApplicationIdentificationV2 {
    #[serde(rename = "lengthOfFollowingData")]
    pub length_of_following_data: u16,
    #[serde(rename = "noOfBorderCrossingRecords")]
    pub no_of_border_crossing_records: u16,
    #[serde(rename = "noOfLoadUnloadRecords")]
    pub no_of_load_unload_records: u16,
    #[serde(rename = "noOfLoadTypeEntryRecords")]
    pub no_of_load_type_entry_records: u16,
    #[serde(rename = "vuConfigurationLengthRange")]
    pub vu_configuration_length_range: u16,
}

impl Readable<DriverCardApplicationIdentificationV2> for DriverCardApplicationIdentificationV2 {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<DriverCardApplicationIdentificationV2> {
        let length_of_following_data = reader.read_u16::<BigEndian>()?;
        let no_of_border_crossing_records = reader.read_u16::<BigEndian>()?;
        let no_of_load_unload_records = reader.read_u16::<BigEndian>()?;
        let no_of_load_type_entry_records = reader.read_u16::<BigEndian>()?;
        let vu_configuration_length_range = reader.read_u16::<BigEndian>()?;

        Ok(Self {
            length_of_following_data,
            no_of_border_crossing_records,
            no_of_load_unload_records,
            no_of_load_type_entry_records,
            vu_configuration_length_range,
        })
    }
}
