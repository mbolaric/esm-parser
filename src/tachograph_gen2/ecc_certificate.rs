use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{ReadableWithParams, Result, gen2::CertificateParams, tacho::CertificateContentType};

#[derive(Debug)]
pub struct EccCertificate {
    pub record_type: CertificateContentType,
    pub record_size: u16,
    pub data: Vec<u8>,
}

impl ReadableWithParams<EccCertificate> for EccCertificate {
    type P = CertificateParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<EccCertificate> {
        let pk_length = params.size;
        let record_type: CertificateContentType = reader.read_u16::<BigEndian>()?.into();
        let record_size = if let Some(size) = pk_length { size } else { reader.read_u8()? as u16 };
        let data = reader.read_into_vec(record_size as u32)?;

        Ok(Self { record_type, record_size, data })
    }
}
