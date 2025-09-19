use serde::Serialize;

use crate::{
    Readable,
    tacho::{NationNumeric, VehicleRegistrationNumber},
};

/// Identification of a vehicle, unique for Europe (VRN and Member State).
#[derive(Debug, Serialize)]
pub struct VehicleRegistrationIdentification {
    #[serde(rename = "vehicleRegistrationNation")]
    pub vehicle_registration_nation: NationNumeric,
    #[serde(rename = "vehicleRegistrationNumber")]
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
