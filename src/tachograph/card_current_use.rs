use serde::Serialize;

use crate::{
    Readable,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug, Serialize)]
pub struct CardCurrentUse {
    pub session_open_time: TimeReal,
    pub session_open_vehicle: VehicleRegistrationIdentification,
}

impl Readable<CardCurrentUse> for CardCurrentUse {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardCurrentUse> {
        let session_open_time = TimeReal::read(reader)?;
        let session_open_vehicle = VehicleRegistrationIdentification::read(reader)?;
        Ok(Self { session_open_time, session_open_vehicle })
    }
}
