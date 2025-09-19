use serde::Serialize;

use crate::{CodePage, Readable, bytes_to_string};

/// Registration number of the vehicle (VRN). The registration number is
/// assigned by the vehicle licensing authority.
#[derive(Debug, Serialize)]
pub struct VehicleRegistrationNumber {
    #[serde(rename = "codePage")]
    pub code_page: CodePage,
    #[serde(rename = "vehicleRegNumber")]
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
