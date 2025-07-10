use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};

use crate::{ReadableWithParams, Result};

#[derive(Debug)]
pub struct CertificateParams {
    pub size: Option<u16>,
}

impl CertificateParams {
    pub fn new(size: Option<u16>) -> Self {
        Self { size }
    }
}

#[derive(Debug)]
pub struct Certificate {
    pub certificate_profile: Option<Vec<u8>>,
    pub data: Vec<u8>,
}

impl ReadableWithParams<Certificate> for Certificate {
    type P = CertificateParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Certificate> {
        if let Some(size) = params.size {
            // FIXME: need to be parsed
            let certificate_profile = reader.read_into_vec(size as u32)?;
            let data = if reader.pos()? >= reader.len()? {
                let mut buff: Vec<u8> = Vec::new();
                reader.read_to_end(&mut buff);
                buff
            } else {
                Vec::new()
            };
            return Ok(Self { certificate_profile: Some(certificate_profile), data });
        } else {
            let data = reader.read_into_vec(reader.len()? as u32)?;
            return Ok(Self { certificate_profile: None, data });
        }
    }
}
