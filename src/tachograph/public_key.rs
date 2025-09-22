use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{ReadableWithParams, Result, tacho::CertificateContentType};

#[derive(Debug)]
pub struct PublicKeyParams {
    pub length: Option<u8>,
}

impl PublicKeyParams {
    pub fn new(length: Option<u8>) -> Self {
        Self { length }
    }

    pub fn empty() -> Self {
        Self { length: None }
    }
}

/// A public RSA key.
/// The Public Key nests two data elements: the standardized domain
/// parameters to be used with the public key in the certificate and the
/// value of the public point.
#[derive(Debug, Serialize)]
pub struct PublicKey {
    #[serde(rename = "recordType")]
    pub record_type: CertificateContentType,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "rsaKeyModulus")]
    pub rsa_key_modulus: Vec<u8>,
}

impl ReadableWithParams<PublicKey> for PublicKey {
    type P = PublicKeyParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<PublicKey> {
        let record_type: CertificateContentType = reader.read_u16::<BigEndian>()?.into();
        let record_size = if params.length.is_some() { params.length.unwrap() } else { reader.read_u8()? };
        let rsa_key_modulus = reader.read_into_vec(record_size as u32)?;
        Ok(Self { record_type, record_size: record_size as u16, rsa_key_modulus })
    }
}
