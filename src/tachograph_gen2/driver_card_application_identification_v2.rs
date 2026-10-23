use binary_data::BigEndian;
use serde::Serialize;

use crate::tacho::CardFileID;
use crate::{Error, Readable, Result};

/// V2 application capacities shared by driver and workshop cards.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2DriverCardApplicationIdentificationV2"))]
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

impl Readable for DriverCardApplicationIdentificationV2 {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> Result<DriverCardApplicationIdentificationV2> {
        let length_of_following_data = reader.read_u16::<BigEndian>()?;
        if length_of_following_data != 8 {
            return Err(Error::InvalidDataParse(format!(
                "Unsupported driver ApplicationIdentificationV2 lengthOfFollowingData: {length_of_following_data}, expected 8"
            )));
        }
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

impl DriverCardApplicationIdentificationV2 {
    pub(crate) fn require<'a>(application: Option<&'a Self>, required_by: &CardFileID) -> Result<&'a Self> {
        application.ok_or_else(|| {
            Error::MissingCardFile(format!("{} (required to parse {required_by})", CardFileID::ApplicationIdentificationV2))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_only_file_requires_v2_application_identification() {
        let error = DriverCardApplicationIdentificationV2::require(None, &CardFileID::BorderCrossings).unwrap_err();
        assert!(matches!(error, Error::MissingCardFile(message)
            if message.contains("ApplicationIdentificationV2") && message.contains("BorderCrossings")));
    }
}
