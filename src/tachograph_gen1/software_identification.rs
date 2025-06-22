use crate::{helpers::vec_u8_to_string, tacho::TimeReal, Readable};

#[derive(Debug)]
pub struct SoftwareIdentification {
    pub software_version: String,
    pub software_installation_date: TimeReal,
}

impl Readable<SoftwareIdentification> for SoftwareIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<SoftwareIdentification> {
        let software_version = vec_u8_to_string(reader.read_into_vec(4)?)?;
        let software_installation_date = TimeReal::read(reader)?;

        Ok(Self {
            software_version,
            software_installation_date,
        })
    }
}
