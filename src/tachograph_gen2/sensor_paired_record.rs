use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result, bytes_to_ia5_fix_string,
    tacho::{ExtendedSerialNumber, TimeReal},
};

#[derive(Debug)]
pub struct SensorPairedRecord {
    pub sensor_serial_number: ExtendedSerialNumber,
    pub sensor_approval_number: String,
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
