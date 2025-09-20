use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, Result,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

/// Information related to the vehicle previously used by a driver when
/// inserting his card in a vehicle unit (Annex 1B requirement 081 and
/// Annex 1C requirement 102).
#[derive(Debug, Serialize)]
pub struct PreviousVehicleInfo {
    #[serde(rename = "vehicleRegistrationIdentification")]
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    #[serde(rename = "cardWithdrawalTime")]
    pub card_withdrawal_time: TimeReal,
    #[serde(rename = "vuGeneration")]
    pub vu_generation: u8,
}

impl Readable<PreviousVehicleInfo> for PreviousVehicleInfo {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<PreviousVehicleInfo> {
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let card_withdrawal_time = TimeReal::read(reader)?;
        let vu_generation = reader.read_u8()?;

        Ok(Self { vehicle_registration_identification, card_withdrawal_time, vu_generation })
    }
}
