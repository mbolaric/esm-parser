use binary_data::{BinSeek, ReadBytes};

use crate::{Readable, Result, helpers::vec_u8_to_string, tacho::TimeReal};

#[derive(Debug)]
pub struct VuSoftwareIdentification {
    pub vu_software_version: String,
    pub vu_soft_installation_date: TimeReal,
}

impl Readable<VuSoftwareIdentification> for VuSoftwareIdentification {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuSoftwareIdentification> {
        let vu_software_version = vec_u8_to_string(reader.read_into_vec(4)?)?;
        let vu_soft_installation_date = TimeReal::read(reader)?;

        Ok(Self { vu_software_version, vu_soft_installation_date })
    }
}
