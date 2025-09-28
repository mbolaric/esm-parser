use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{ExtendedSerialNumber, TimeReal},
};

const SENSOR_APPROVAL_NUMBER_LENGTH: u32 = 8;

/// Information, stored in a vehicle unit, related to the identification of the
/// motion sensor paired with the vehicle unit (Annex 1B requirement 079).
#[derive(Debug, Serialize)]
pub struct SensorPaired {
    #[serde(rename = "sensorSerialNumber")]
    pub sensor_serial_number: ExtendedSerialNumber,
    #[serde(rename = "sensorApprovalNumber")]
    pub sensor_approval_number: String,
    #[serde(rename = "sensorPairingDateFirst")]
    pub sensor_pairing_date_first: TimeReal,
}

impl Readable<SensorPaired> for SensorPaired {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SensorPaired> {
        let sensor_serial_number = ExtendedSerialNumber::read(reader)?;
        let sensor_approval_number = bytes_to_ia5_fix_string(&reader.read_into_vec(SENSOR_APPROVAL_NUMBER_LENGTH)?)?;
        let sensor_pairing_date_first = TimeReal::read(reader)?;

        Ok(Self { sensor_serial_number, sensor_approval_number, sensor_pairing_date_first })
    }
}
