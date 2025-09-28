use serde::Serialize;

use crate::{Readable, bytes_to_ia5_fix_string, tacho::TimeReal};

const VU_SOFTWARE_VERSION_LENGTH: u32 = 4;

#[derive(Debug, Serialize)]
pub struct VuSoftwareIdentification {
    #[serde(rename = "vuSoftwareVersion")]
    pub vu_software_version: String,
    #[serde(rename = "vuSoftInstallationDate")]
    pub vu_software_installation_date: TimeReal,
}

impl Readable<VuSoftwareIdentification> for VuSoftwareIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuSoftwareIdentification> {
        let vu_software_version = bytes_to_ia5_fix_string(&reader.read_into_vec(VU_SOFTWARE_VERSION_LENGTH)?)?;
        let vu_software_installation_date = TimeReal::read(reader)?;

        Ok(Self { vu_software_version, vu_software_installation_date })
    }
}
