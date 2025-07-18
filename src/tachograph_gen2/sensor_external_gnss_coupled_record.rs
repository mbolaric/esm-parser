use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result, bytes_to_ia5_fix_string,
    tacho::{ExtendedSerialNumber, TimeReal},
};

#[derive(Debug)]
pub struct SensorExternalGNSSCoupledRecord {
    pub sensor_external_gnss_serial_number: ExtendedSerialNumber,
    pub sensor_external_gnss_approval_number: String,
    pub sensor_gnss_coupling_date: TimeReal,
}

impl Readable<SensorExternalGNSSCoupledRecord> for SensorExternalGNSSCoupledRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<SensorExternalGNSSCoupledRecord> {
        let sensor_external_gnss_serial_number = ExtendedSerialNumber::read(reader)?;
        let sensor_external_gnss_approval_number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
        let sensor_gnss_coupling_date = TimeReal::read(reader)?;

        Ok(Self { sensor_external_gnss_serial_number, sensor_external_gnss_approval_number, sensor_gnss_coupling_date })
    }
}
