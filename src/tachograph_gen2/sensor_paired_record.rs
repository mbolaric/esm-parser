use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result, bytes_to_ia5_fix_string,
    tacho::{ExtendedSerialNumber, TimeReal},
};

/// Information, stored in a vehicle unit, related to the identification of a
/// motion sensor paired with the vehicle unit (Annex 1C requirement 97).
#[derive(Debug, Serialize)]
pub struct SensorPairedRecord {
    #[serde(rename = "sensorSerialNumber")]
    pub sensor_serial_number: ExtendedSerialNumber,
    #[serde(rename = "sensorApprovalNumber")]
    pub sensor_approval_number: String,
    #[serde(rename = "sensorPairingDate")]
    pub sensor_pairing_date: TimeReal,
}

impl Readable<SensorPairedRecord> for SensorPairedRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SensorPairedRecord> {
        let sensor_serial_number = ExtendedSerialNumber::read(reader)?;
        let sensor_approval_number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
        let sensor_pairing_date = TimeReal::read(reader)?;
        Ok(Self { sensor_serial_number, sensor_approval_number, sensor_pairing_date })
    }
}
