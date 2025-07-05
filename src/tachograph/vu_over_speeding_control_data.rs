use crate::{Readable, tacho::TimeReal};

#[derive(Debug)]
pub struct VuOverSpeedingControlData {
    pub last_overspeed_control_time: TimeReal,
    pub first_overspeed_since: TimeReal,
    pub number_of_overspeed_since: u8,
}

impl Readable<VuOverSpeedingControlData> for VuOverSpeedingControlData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingControlData> {
        let last_overspeed_control_time = TimeReal::read(reader)?;
        let first_overspeed_since = TimeReal::read(reader)?;
        let number_of_overspeed_since = reader.read_u8()?;
        Ok(Self { last_overspeed_control_time, first_overspeed_since, number_of_overspeed_since })
    }
}
