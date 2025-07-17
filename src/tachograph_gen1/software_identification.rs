use crate::{Readable, bytes_to_ia5_fix_string, tacho::TimeReal};

#[derive(Debug)]
pub struct SoftwareIdentification {
    pub software_version: String,
    pub software_installation_date: TimeReal,
}

impl Readable<SoftwareIdentification> for SoftwareIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SoftwareIdentification> {
        let software_version = bytes_to_ia5_fix_string(&reader.read_into_vec(4)?)?;
        let software_installation_date = TimeReal::read(reader)?;

        Ok(Self { software_version, software_installation_date })
    }
}
