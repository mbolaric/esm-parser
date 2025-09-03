use binary_data::BigEndian;

use crate::Readable;

#[derive(Debug)]
pub struct CompanyCardApplicationIdentificationV2 {
    pub length_of_following_data: u16,
    pub vu_configuration_length_range: u16,
}

impl Readable<CompanyCardApplicationIdentificationV2> for CompanyCardApplicationIdentificationV2 {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CompanyCardApplicationIdentificationV2> {
        let length_of_following_data = reader.read_u16::<BigEndian>()?;
        let vu_configuration_length_range = reader.read_u16::<BigEndian>()?;

        Ok(Self { length_of_following_data, vu_configuration_length_range })
    }
}
