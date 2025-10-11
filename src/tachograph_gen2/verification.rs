use log::debug;

use crate::{
    Error, Result,
    tacho::{CardFileID, CardFilesMap, TimeReal, VerifyResult},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CertificateTag {
    ApplicationTemplate = 0x7F81,
    CertificateBody = 0x7FAE,
    CertificateProfileIdentifier = 0x5F19,
    CertificateAuthorityReference = 0x42,
    CertificateHolderAuthorisation = 0x5F3C,
    Extensions = 0x7FA9,
    DomainParameters = 0x06,
    PublicPoint = 0x86,
    CertificateHolderReference = 0x5F20,
    CertificateEffectiveDate = 0x5F25,
    CertificateExpirationDate = 0x5F24,
    CertificateSignature = 0x5F37,
    Unknown = 0xFFFF,
}

impl From<u32> for CertificateTag {
    fn from(value: u32) -> Self {
        match value {
            0x7F81 => Self::ApplicationTemplate,
            0x7FAE => Self::CertificateBody,
            0x5F19 => Self::CertificateProfileIdentifier,
            0x42 => Self::CertificateAuthorityReference,
            0x5F3C => Self::CertificateHolderAuthorisation,
            0x7FA9 => Self::Extensions,
            0x06 => Self::DomainParameters,
            0x86 => Self::PublicPoint,
            0x5F20 => Self::CertificateHolderReference,
            0x5F25 => Self::CertificateEffectiveDate,
            0x5F24 => Self::CertificateExpirationDate,
            0x5F37 => Self::CertificateSignature,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_profile_identifier: u32,
    pub certificate_authority_reference: Option<Vec<u8>>,
    pub certificate_holder_authorisation: Option<Vec<u8>>,
    pub domain_parameters: Option<String>,
    pub public_point: Option<Vec<u8>>,
    pub certificate_holder_reference: Option<Vec<u8>>,
    pub certificate_effective_date: Option<TimeReal>,
    pub certificate_expiration_date: Option<TimeReal>,
    pub certificate_body: Option<Vec<u8>>,
    pub certificate_signature: Option<Vec<u8>>,
}

impl Certificate {
    pub fn from_bytes(data: &[u8; 205]) -> Result<Self> {
        let mut cert = Certificate {
            certificate_profile_identifier: 0,
            certificate_authority_reference: None,
            certificate_holder_authorisation: None,
            domain_parameters: None,
            public_point: None,
            certificate_holder_reference: None,
            certificate_effective_date: None,
            certificate_expiration_date: None,
            certificate_body: None,
            certificate_signature: None,
        };
        Ok(Certificate::parse(data, &mut cert))
    }

    #[allow(dead_code)]
    fn read_tlv(data: &[u8; 205], pos: &mut usize) -> Option<(u32, usize)> {
        if *pos >= data.len() {
            return None;
        }

        let mut tag = data[*pos] as u32;

        // Handle multi-byte tag
        if tag & 0x1F == 0x1F {
            *pos += 1;
            tag = (tag << 8) + data[*pos] as u32;
        }
        *pos += 1;

        // Parse length
        let data_part_len: usize;
        if data[*pos] & 0x80 == 0x80 {
            let int_len = (data[*pos] & 0x7F) as usize;
            data_part_len = Certificate::to_u32(data.as_slice(), *pos + 1, int_len) as usize;
            *pos += int_len;
        } else {
            data_part_len = data[*pos] as usize;
        }
        *pos += 1;

        Some((tag, data_part_len))
    }

    fn parse(data: &[u8; 205], cert: &mut Certificate) -> Certificate {
        let mut i = 0;
        // while let Some((tag, data_part_len)) = Certificate::read_tlv(&data, &mut i) {
        //     let tag_enum = CertificateTag::from(tag);
        // }

        while i < data.len() {
            let start = i;
            let mut tag = data[i] as u32;

            // Handle multi-byte tag
            if tag & 0x1F == 0x1F {
                i += 1;
                tag = (tag << 8) + data[i] as u32;
            }
            i += 1;

            // Parse length
            let data_part_len: usize;
            if data[i] & 0x80 == 0x80 {
                let int_len = (data[i] & 0x7F) as usize;
                data_part_len = Certificate::to_u32(data.as_slice(), i + 1, int_len) as usize;
                i += int_len;
            } else {
                data_part_len = data[i] as usize;
            }
            i += 1;

            // Use enum instead of raw numbers
            let tag_enum = CertificateTag::from(tag);

            match tag_enum {
                CertificateTag::ApplicationTemplate => { /* ignore */ }
                CertificateTag::Extensions => { /* ignore */ }
                CertificateTag::CertificateBody => {
                    cert.certificate_body = Some(data[start..i + data_part_len].to_vec());
                }
                CertificateTag::CertificateProfileIdentifier => {
                    cert.certificate_profile_identifier = Certificate::to_u32(data.as_slice(), i, data_part_len);
                }
                CertificateTag::CertificateAuthorityReference => {
                    cert.certificate_authority_reference = Some(data[i..i + data_part_len].to_vec());
                }
                CertificateTag::CertificateHolderAuthorisation => {
                    cert.certificate_holder_authorisation = Some(data[i..i + data_part_len].to_vec());
                }
                CertificateTag::DomainParameters => {
                    cert.domain_parameters = Some(Certificate::to_object_identifier(&data[i..i + data_part_len]));
                }
                CertificateTag::PublicPoint => {
                    cert.public_point = Some(data[i..i + data_part_len].to_vec());
                }
                CertificateTag::CertificateHolderReference => {
                    cert.certificate_holder_reference = Some(data[i..i + data_part_len].to_vec());
                }
                CertificateTag::CertificateEffectiveDate => {
                    cert.certificate_effective_date = Some(Certificate::to_time_real(&data[i..i + data_part_len]));
                }
                CertificateTag::CertificateExpirationDate => {
                    cert.certificate_expiration_date = Some(Certificate::to_time_real(&data[i..i + data_part_len]));
                }
                CertificateTag::CertificateSignature => {
                    cert.certificate_signature = Some(data[i..i + data_part_len].to_vec());
                }
                CertificateTag::Unknown => {
                    debug!("Unknown tag: {:#X}", tag);
                }
            }

            i += data_part_len;
        }

        cert.clone()
    }

    fn to_u32(data: &[u8], offset: usize, len: usize) -> u32 {
        data[offset..offset + len].iter().fold(0u32, |acc, &b| (acc << 8) | b as u32)
    }

    fn to_object_identifier(bytes: &[u8]) -> String {
        // basic OID decode placeholder
        bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(".")
    }

    fn to_time_real(data: &[u8]) -> TimeReal {
        TimeReal::new(Certificate::to_u32(data, 0, data.len()))
    }
}

pub fn verify(data_files: &CardFilesMap, _erca_pk: &[u8; 205]) -> Result<VerifyResult> {
    let ic = data_files.get(&CardFileID::IC);
    let icc = data_files.get(&CardFileID::ICC);
    if ic.is_none() || icc.is_none() {
        return Ok(VerifyResult { status: crate::tacho::VerifyResultStatus::Unsigned, result: Vec::new() });
    }
    let ca_cert_file =
        data_files.get(&CardFileID::CACertificate).ok_or(Error::VerifyError("Missing CA Certificate.".to_string()))?;
    let _card_cert_file = data_files
        .get(&CardFileID::CardSignCertificate)
        .ok_or(Error::VerifyError("Missing Card Sign Certificate.".to_string()))?;

    let ca_signature = ca_cert_file.data.as_ref().ok_or_else(|| Error::VerifyError("Missing Certificate Data.".to_string()))?;

    let ca_signature_array: &[u8; 205] = ca_signature
        .as_slice()
        .try_into()
        .map_err(|_| Error::VerifyError("Invalid signature length in Certificate.".to_string()))?;

    let ca_certificate = Certificate::from_bytes(ca_signature_array);
    debug!("verify - CA Certificate: {:?}", ca_certificate);

    Err(Error::NotImplemented)
}
