use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{CodePage, Readable, Result, bytes_to_string, tacho::TimeReal};

/// Information, stored in a vehicle unit, related to the software installed.
#[derive(Debug, Serialize)]
pub struct VuSoftwareIdentification {
    #[serde(rename = "vuSoftwareVersion")]
    pub vu_software_version: String,
    #[serde(rename = "vuSoftInstallationDate")]
    pub vu_soft_installation_date: TimeReal,
}

impl Readable<VuSoftwareIdentification> for VuSoftwareIdentification {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuSoftwareIdentification> {
        let vu_software_version = bytes_to_string(&reader.read_into_vec(4)?, &CodePage::IsoIec8859_1);
        let vu_soft_installation_date = TimeReal::read(reader)?;

        Ok(Self { vu_software_version, vu_soft_installation_date })
    }
}
