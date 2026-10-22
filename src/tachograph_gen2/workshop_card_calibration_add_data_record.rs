use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::LoadType;
use crate::tacho::{NationNumeric, TimeReal};
use crate::{Readable, Result, bytes_to_ia5_fix_string};

/// Additional calibration data stored in a V2 workshop card (Appendix 1, section 2.234c).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2WorkshopCardCalibrationAddDataRecord"))]
pub struct WorkshopCardCalibrationAddDataRecord {
    #[serde(rename = "oldTimeValue")]
    pub old_time_value: TimeReal,
    #[serde(rename = "vehicleIdentificationNumber")]
    pub vehicle_identification_number: String,
    #[serde(rename = "byDefaultLoadType")]
    pub by_default_load_type: LoadType,
    #[serde(rename = "calibrationCountry")]
    pub calibration_country: NationNumeric,
    #[serde(rename = "calibrationCountryTimestamp")]
    pub calibration_country_timestamp: TimeReal,
}

impl Readable for WorkshopCardCalibrationAddDataRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
        let old_time_value = TimeReal::read(reader)?;
        let vehicle_identification_number = bytes_to_ia5_fix_string(&reader.read_into_vec(17)?)?;
        let by_default_load_type = reader.read_u8()?.into();
        let calibration_country = reader.read_u8()?.into();
        let calibration_country_timestamp = TimeReal::read(reader)?;
        Ok(Self {
            old_time_value,
            vehicle_identification_number,
            by_default_load_type,
            calibration_country,
            calibration_country_timestamp,
        })
    }
}
