use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct PreviousVehicleInfo {
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub card_withdrawal_time: TimeReal,
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
