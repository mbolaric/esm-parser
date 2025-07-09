use crate::{
    Readable,
    tacho::{TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct PreviousVehicleInfo {
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub card_withdrawal_time: TimeReal,
}

impl Readable<PreviousVehicleInfo> for PreviousVehicleInfo {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<PreviousVehicleInfo> {
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let card_withdrawal_time = TimeReal::read(reader)?;
        Ok(Self { vehicle_registration_identification, card_withdrawal_time })
    }
}
