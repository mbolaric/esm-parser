use crate::common::Readable;
use crate::helpers::vec_u8_to_string;
use crate::tacho::NationNumericCode;

#[derive(Debug)]
pub struct VehicleRegistrationIdentification {
    pub vehicle_registration_nation: NationNumericCode,
    pub code_page: u8,
    pub registration_number: String,
}

impl VehicleRegistrationIdentification {}

impl Readable<VehicleRegistrationIdentification> for VehicleRegistrationIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VehicleRegistrationIdentification> {
        let vehicle_registration_nation: NationNumericCode = reader.read_u8()?.into();
        let code_page = reader.read_u8()?;
        let registration_number = vec_u8_to_string(reader.read_into_vec(13)?)?
            .trim()
            .to_owned();

        Ok(Self {
            vehicle_registration_nation,
            code_page,
            registration_number,
        })
    }
}
