use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    tacho::{CertificateContentType, EquipmentType},
};

#[derive(Debug)]
pub struct CertificateHolderAuthorisation {
    pub record_type: CertificateContentType,
    pub record_size: u16,
    pub tachograph_application_id: Vec<u8>,
    pub equipment_type: EquipmentType,
}

impl Readable<CertificateHolderAuthorisation> for CertificateHolderAuthorisation {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CertificateHolderAuthorisation> {
        let record_type: CertificateContentType = (reader.read_u16::<BigEndian>()?).into();
        let record_size = reader.read_u8()? as u16;
        let tachograph_application_id = reader.read_into_vec(6)?;
        let equipment_type: EquipmentType = reader.read_u8()?.into();

        Ok(Self { record_type, record_size, tachograph_application_id, equipment_type })
    }
}
