use serde::Serialize;

use crate::{CodePage, Readable, bytes_to_string};

/// Registration number of the vehicle (VRN). The registration number is
/// assigned by the vehicle licensing authority.
#[derive(Debug)]
pub struct VehicleRegistrationNumber {
    pub code_page: CodePage,
    pub vehicle_reg_number: String,
}

impl Readable<VehicleRegistrationNumber> for VehicleRegistrationNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VehicleRegistrationNumber> {
        let code_page: CodePage = reader.read_u8()?.into();
        let mut vehicle_reg_number = bytes_to_string(&reader.read_into_vec(13)?, &code_page);
        vehicle_reg_number = if code_page == CodePage::Invalid { "".to_owned() } else { vehicle_reg_number };
        Ok(Self { code_page, vehicle_reg_number })
    }
}

impl Serialize for VehicleRegistrationNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.vehicle_reg_number)
    }
}
