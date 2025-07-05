use crate::{
    Readable,
    tacho::{NationNumeric, VehicleRegistrationNumber},
};

#[derive(Debug)]
pub struct VehicleRegistrationIdentification {
    pub nation_numeric: NationNumeric,
    pub vehicle_registration_number: VehicleRegistrationNumber,
}

impl Readable<VehicleRegistrationIdentification> for VehicleRegistrationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VehicleRegistrationIdentification> {
        let nation_numeric = reader.read_u8()?.into();
        let vehicle_registration_number = VehicleRegistrationNumber::read(reader)?;

        Ok(Self { nation_numeric, vehicle_registration_number })
    }
}
