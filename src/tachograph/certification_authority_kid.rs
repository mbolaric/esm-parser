use binary_data::BigEndian;

use crate::{Readable, bytes_to_ia5_fix_string, tacho::NationNumeric};

#[derive(Debug)]
pub struct CertificationAuthorityKid {
    pub nation_numeric: NationNumeric,
    pub nation_alpha: String,
    pub key_serial_number: u8,
    pub additional_info: u16,
    pub ca_identifier: u8,
}

impl Readable<CertificationAuthorityKid> for CertificationAuthorityKid {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CertificationAuthorityKid> {
        let nation_numeric: NationNumeric = reader.read_u8()?.into();
        let nation_alpha = bytes_to_ia5_fix_string(&reader.read_into_vec(3)?)?;
        let key_serial_number = reader.read_u8()?;
        let additional_info = reader.read_u16::<BigEndian>()?;
        let ca_identifier = reader.read_u8()?;

        Ok(Self { nation_numeric, nation_alpha, key_serial_number, additional_info, ca_identifier })
    }
}
