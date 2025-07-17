use crate::{CodePage, Readable, bytes_to_string};

#[derive(Debug)]
pub struct VehicleRegistrationNumber {
    pub code_page: CodePage,
    pub registration_number: String,
}

impl Readable<VehicleRegistrationNumber> for VehicleRegistrationNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VehicleRegistrationNumber> {
        let code_page: CodePage = reader.read_u8()?.into();
        let mut registration_number = bytes_to_string(&reader.read_into_vec(13)?, &code_page);
        registration_number = if code_page == CodePage::Invalid { "".to_owned() } else { registration_number.trim().to_owned() };
        Ok(Self { code_page, registration_number })
    }
}
