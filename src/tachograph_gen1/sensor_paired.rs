use crate::{gen1::SerialNumber, helpers::vec_u8_to_string, tacho::TimeReal, Readable};

#[derive(Debug)]
pub struct SensorPaired {
    pub serial_number: SerialNumber,
    pub sensor_approval_number: String,
    pub sensor_pairing_date_first: TimeReal,
}

impl Readable<SensorPaired> for SensorPaired {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<SensorPaired> {
        let serial_number = SerialNumber::read(reader)?;
        let sensor_approval_number = vec_u8_to_string(reader.read_into_vec(8)?)?;
        let sensor_pairing_date_first = TimeReal::read(reader)?;

        Ok(Self {
            serial_number,
            sensor_approval_number,
            sensor_pairing_date_first,
        })
    }
}
