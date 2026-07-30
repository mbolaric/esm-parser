use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::{ExtendedSerialNumber, TimeReal};
use crate::{Readable, Result, bytes_to_ia5_fix_string};

/// Information, stored in a vehicle unit, related to the identification of the
/// external GNSS facility coupled with the vehicle unit (Annex 1C requirement 100).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2SensorExternalGNSSCoupledRecord"))]
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
