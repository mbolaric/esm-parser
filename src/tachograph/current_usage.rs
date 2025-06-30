use crate::{
    Readable,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct CurrentUsage {
    pub session_open_time: TimeReal,
    pub session_open_vehicle: VehicleRegistrationIdentification,
}

impl Readable<CurrentUsage> for CurrentUsage {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CurrentUsage> {
        let session_open_time = TimeReal::read(reader)?;
        let session_open_vehicle = VehicleRegistrationIdentification::read(reader)?;
        Ok(Self { session_open_time, session_open_vehicle })
    }
}
