use serde::Serialize;

use crate::{
    Readable,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

/// Information about the actual usage of the card (Annex 1C requirement
/// 273, 298, 326, and 349).
#[derive(Debug, Serialize)]
pub struct CardCurrentUse {
    #[serde(rename = "sessionOpenTime")]
    pub session_open_time: TimeReal,
    #[serde(rename = "sessionOpenVehicle")]
    pub session_open_vehicle: VehicleRegistrationIdentification,
}

impl Readable<CardCurrentUse> for CardCurrentUse {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardCurrentUse> {
        let session_open_time = TimeReal::read(reader)?;
        let session_open_vehicle = VehicleRegistrationIdentification::read(reader)?;
        Ok(Self { session_open_time, session_open_vehicle })
    }
}
