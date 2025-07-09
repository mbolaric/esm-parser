use std::u8;

use crate::{Readable, helpers::vec_u8_to_string};

#[derive(Debug)]
pub struct VehicleRegistrationNumber {
    pub code_page: u8,
    pub registration_number: String,
}

impl Readable<VehicleRegistrationNumber> for VehicleRegistrationNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VehicleRegistrationNumber> {
        let code_page = reader.read_u8()?;
        let mut registration_number = vec_u8_to_string(reader.read_into_vec(13)?)?;
        registration_number = if code_page == u8::MAX { "".to_owned() } else { registration_number.trim().to_owned() };
        Ok(Self { code_page, registration_number })
    }
}
