use crate::{
    Readable,
    tacho::{NationNumeric, VehicleRegistrationNumber},
};

#[derive(Debug)]
pub struct VehicleRegistrationIdentification {
    pub vehicle_registration_nation: NationNumeric,
    pub vehicle_registration_number: VehicleRegistrationNumber,
}

impl Readable<VehicleRegistrationIdentification> for VehicleRegistrationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VehicleRegistrationIdentification> {
        let nation_numeric: NationNumeric = reader.read_u8()?.into();
        let vehicle_registration_number = VehicleRegistrationNumber::read(reader)?;

        Ok(Self { vehicle_registration_nation: nation_numeric, vehicle_registration_number })
    }
}
