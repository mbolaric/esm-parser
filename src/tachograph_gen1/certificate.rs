use crate::Readable;

#[derive(Debug)]
pub struct Certificate {
    pub signature: Vec<u8>,
    pub public_key_remainder: Vec<u8>,
    pub certification_authority_reference: Vec<u8>,
}

impl Readable<Certificate> for Certificate {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Certificate> {
        // Size = 194
        let signature = reader.read_into_vec(128)?;
        let public_key_remainder = reader.read_into_vec(58)?;
        // https://dtc.jrc.ec.europa.eu/dtc_public_key_certificates_dt.php.html
        let certification_authority_reference = reader.read_into_vec(8)?;

        Ok(Self { signature, public_key_remainder, certification_authority_reference })
    }
}
