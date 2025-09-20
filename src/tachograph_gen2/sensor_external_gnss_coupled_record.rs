use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result, bytes_to_ia5_fix_string,
    tacho::{ExtendedSerialNumber, TimeReal},
};

/// Information, stored in a vehicle unit, related to the identification of the
/// external GNSS facility coupled with the vehicle unit (Annex 1C requirement 100).
#[derive(Debug, Serialize)]
pub struct SensorExternalGNSSCoupledRecord {
    #[serde(rename = "sensorSerialNumber")]
    pub sensor_serial_number: ExtendedSerialNumber,
    #[serde(rename = "sensorApprovalNumber")]
    pub sensor_approval_number: String,
    #[serde(rename = "sensorCouplingDate")]
    pub sensor_coupling_date: TimeReal,
}

impl Readable<SensorExternalGNSSCoupledRecord> for SensorExternalGNSSCoupledRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SensorExternalGNSSCoupledRecord> {
        let sensor_serial_number = ExtendedSerialNumber::read(reader)?;
        let sensor_approval_number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
        let sensor_coupling_date = TimeReal::read(reader)?;

        Ok(Self { sensor_serial_number, sensor_approval_number, sensor_coupling_date })
    }
}
