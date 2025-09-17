use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    gen2::{CertificateProfile, CertificateProfileParams},
};

#[derive(Debug)]
pub struct CertificateParams {
    pub size: Option<u16>,
}

impl CertificateParams {
    pub fn new(size: Option<u16>) -> Self {
        Self { size }
    }
}

#[derive(Debug, Serialize)]
pub struct Certificate {
    pub certificate_profile: Option<CertificateProfile>,
    pub data: Vec<u8>,
}

impl ReadableWithParams<Certificate> for Certificate {
    type P = CertificateParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Certificate> {
        if let Some(size) = params.size {
            let certificate_profile = CertificateProfile::read(reader, &CertificateProfileParams::new(size))?;
            let data = if !reader.is_eof() {
                let mut buff: Vec<u8> = Vec::new();
                let _ = reader.read_to_end(&mut buff);
                buff
            } else {
                Vec::new()
            };
            Ok(Self { certificate_profile: Some(certificate_profile), data })
        } else {
            let data = reader.read_into_vec(reader.len()? as u32)?;
            Ok(Self { certificate_profile: None, data })
        }
    }
}
