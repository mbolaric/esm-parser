use binary_data::{BinSeek, ReadBytes};
use serde::{Serialize, Serializer};

use crate::tacho::{CertificateContentType, TimeReal};
use crate::{Error, Readable, Result};

pub const GEN2_CERTIFICATE_SIZE: usize = 205;
pub const CAR_SIZE: usize = 8;
pub const CHR_SIZE: usize = 8;
pub const CHA_SIZE: usize = 7;
pub const CPI_VERSION_1: u8 = 0x00;
pub const ECDSA_P256_SIGNATURE_SIZE: usize = 64;
pub const SEC1_UNCOMPRESSED_P256_POINT_SIZE: usize = 65;
pub const BRAINPOOL_P256_R1_OID: &str = "1.3.36.3.3.2.8.1.1.7";

mod serde_arr {
    use super::*;

    pub fn serialize<S, const N: usize>(arr: &[u8; N], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        arr.as_slice().serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct CvcTlv<'a> {
    pub tag: u16,
    pub encoded: &'a [u8],
    pub value: &'a [u8],
}

pub fn parse_tlv(input: &[u8]) -> Result<CvcTlv<'_>> {
    let Some(&first_tag_byte) = input.first() else {
        return Err(Error::VerifyError("Unexpected end of CVC data while reading a tag.".to_string()));
    };

    let (tag, tag_len) = if first_tag_byte & 0x1F == 0x1F {
        let Some(&second_tag_byte) = input.get(1) else {
            return Err(Error::VerifyError("Truncated multi-byte CVC tag.".to_string()));
        };
        if second_tag_byte & 0x80 != 0 {
            return Err(Error::VerifyError("Unsupported CVC tag with more than two bytes.".to_string()));
        }
        (((first_tag_byte as u16) << 8) | second_tag_byte as u16, 2)
    } else {
        (first_tag_byte as u16, 1)
    };

    let Some(&first_length_byte) = input.get(tag_len) else {
        return Err(Error::VerifyError("Unexpected end of CVC data while reading a length.".to_string()));
    };

    let (value_len, length_len) = if first_length_byte & 0x80 == 0 {
        (first_length_byte as usize, 1)
    } else {
        let length_byte_count = (first_length_byte & 0x7F) as usize;
        if length_byte_count == 0 {
            return Err(Error::VerifyError("Indefinite CVC lengths are not supported.".to_string()));
        }
        if length_byte_count > core::mem::size_of::<usize>() {
            return Err(Error::VerifyError("CVC length is too large.".to_string()));
        }
        let length_bytes = input
            .get(tag_len + 1..tag_len + 1 + length_byte_count)
            .ok_or_else(|| Error::VerifyError("Truncated long-form CVC length.".to_string()))?;
        if length_bytes.first() == Some(&0) {
            return Err(Error::VerifyError("CVC length is not minimally encoded.".to_string()));
        }
        let value_len = length_bytes.iter().try_fold(0usize, |acc, &byte| {
            acc.checked_shl(8)
                .and_then(|value| value.checked_add(byte as usize))
                .ok_or_else(|| Error::VerifyError("CVC length overflows usize.".to_string()))
        })?;
        if value_len < 128 {
            return Err(Error::VerifyError("CVC long-form length is not minimally encoded.".to_string()));
        }
        (value_len, 1 + length_byte_count)
    };

    let header_len = tag_len + length_len;
    let encoded_len =
        header_len.checked_add(value_len).ok_or_else(|| Error::VerifyError("CVC tag length overflows usize.".to_string()))?;
    let encoded = input
        .get(..encoded_len)
        .ok_or_else(|| Error::VerifyError("CVC tag value exceeds the certificate boundary.".to_string()))?;

    Ok(CvcTlv { tag, encoded, value: &encoded[header_len..] })
}

pub fn take_tlv<'a>(input: &mut &'a [u8]) -> Result<CvcTlv<'a>> {
    let tlv = parse_tlv(input)?;
    *input = &input[tlv.encoded.len()..];
    Ok(tlv)
}

pub fn expect_tlv<'a>(input: &mut &'a [u8], expected_tag: u16, field: &str) -> Result<CvcTlv<'a>> {
    let tlv = take_tlv(input)?;
    if tlv.tag != expected_tag {
        return Err(Error::VerifyError(format!("Expected {field} tag {expected_tag:#06X}, found {:#06X}.", tlv.tag)));
    }
    Ok(tlv)
}

pub fn fixed_value<const N: usize>(tlv: &CvcTlv<'_>, field: &str) -> Result<[u8; N]> {
    tlv.value
        .try_into()
        .map_err(|_| Error::VerifyError(format!("Invalid {field} size: expected {N} bytes, found {} bytes.", tlv.value.len())))
}

pub fn oid_from_der(bytes: &[u8]) -> Result<String> {
    let Some(&first) = bytes.first() else {
        return Err(Error::VerifyError("Domain parameters OID is empty.".to_string()));
    };

    let (first_component, second_component) = match first {
        0..=39 => (0, first as u64),
        40..=79 => (1, (first - 40) as u64),
        _ => (2, (first - 80) as u64),
    };
    let mut parts = vec![first_component.to_string(), second_component.to_string()];
    let mut index = 1;
    while index < bytes.len() {
        let mut value = 0u64;
        let mut octet_count = 0;
        loop {
            let Some(&byte) = bytes.get(index) else {
                return Err(Error::VerifyError("Truncated sub-identifier in domain parameters OID.".to_string()));
            };
            index += 1;
            octet_count += 1;
            if octet_count > 10 {
                return Err(Error::VerifyError("Sub-identifier in domain parameters OID is too large.".to_string()));
            }
            value = (value << 7) | ((byte & 0x7F) as u64);
            if byte & 0x80 == 0 {
                break;
            }
        }
        parts.push(value.to_string());
    }

    Ok(parts.join("."))
}

/// A Card Verifiable Certificate (CVC) as defined in ISO/IEC 7816-8 and Annex 1C Appendix 11.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2CardVerifiableCertificate"))]
pub struct CardVerifiableCertificate {
    #[serde(rename = "certificateAuthorityReference", with = "serde_arr")]
    #[cfg_attr(feature = "typescript", ts(type = "Array<number>"))]
    pub certificate_authority_reference: [u8; CAR_SIZE],
    #[serde(rename = "certificateHolderAuthorisation", with = "serde_arr")]
    #[cfg_attr(feature = "typescript", ts(type = "Array<number>"))]
    pub certificate_holder_authorisation: [u8; CHA_SIZE],
    #[serde(rename = "domainParameters")]
    pub domain_parameters: String,
    #[serde(rename = "publicPoint", with = "serde_arr")]
    #[cfg_attr(feature = "typescript", ts(type = "Array<number>"))]
    pub public_point: [u8; SEC1_UNCOMPRESSED_P256_POINT_SIZE],
    #[serde(rename = "certificateHolderReference", with = "serde_arr")]
    #[cfg_attr(feature = "typescript", ts(type = "Array<number>"))]
    pub certificate_holder_reference: [u8; CHR_SIZE],
    #[serde(rename = "certificateEffectiveDate")]
    pub certificate_effective_date: TimeReal,
    #[serde(rename = "certificateExpirationDate")]
    pub certificate_expiration_date: TimeReal,
    #[serde(rename = "certificateBody")]
    pub certificate_body: Vec<u8>,
    #[serde(rename = "certificateSignature", with = "serde_arr")]
    #[cfg_attr(feature = "typescript", ts(type = "Array<number>"))]
    pub certificate_signature: [u8; ECDSA_P256_SIGNATURE_SIZE],
}

impl CardVerifiableCertificate {
    pub fn from_slice(data: &[u8]) -> Result<Self> {
        let outer = parse_tlv(data)?;
        if outer.tag != CertificateContentType::ECCCertificate as u16 {
            return Err(Error::VerifyError("Gen2 certificate does not start with an ECC Certificate tag.".to_string()));
        }
        if outer.encoded.len() != data.len() {
            return Err(Error::VerifyError("Trailing bytes found after the Gen2 ECC Certificate.".to_string()));
        }

        let mut outer_value = outer.value;
        let certificate_body =
            expect_tlv(&mut outer_value, CertificateContentType::ECCCertificateBody as u16, "ECC Certificate Body")?;
        let certificate_signature =
            expect_tlv(&mut outer_value, CertificateContentType::CertificateSignature as u16, "ECC Certificate Signature")?;
        if !outer_value.is_empty() {
            return Err(Error::VerifyError("Unexpected fields after the Gen2 certificate signature.".to_string()));
        }

        let mut body_value = certificate_body.value;
        let profile = expect_tlv(
            &mut body_value,
            CertificateContentType::CertificateProfileIdentifier as u16,
            "Certificate Profile Identifier",
        )?;
        let certificate_profile_identifier = fixed_value::<1>(&profile, "Certificate Profile Identifier")?[0];
        if certificate_profile_identifier != CPI_VERSION_1 {
            return Err(Error::VerifyError(format!(
                "Unsupported Gen2 certificate profile identifier: {certificate_profile_identifier:#04X}."
            )));
        }

        let car = expect_tlv(
            &mut body_value,
            CertificateContentType::CertificateAuthorityReference as u16,
            "Certificate Authority Reference",
        )?;
        let certificate_authority_reference = fixed_value::<CAR_SIZE>(&car, "Certificate Authority Reference")?;

        let cha = expect_tlv(
            &mut body_value,
            CertificateContentType::CertificateHolderAuthorisation as u16,
            "Certificate Holder Authorisation",
        )?;
        let certificate_holder_authorisation = fixed_value::<CHA_SIZE>(&cha, "Certificate Holder Authorisation")?;

        let public_key = expect_tlv(&mut body_value, CertificateContentType::PublicKey as u16, "Public Key")?;
        let (domain_parameters, public_point) = Self::parse_public_key(public_key.value)?;

        let chr = expect_tlv(
            &mut body_value,
            CertificateContentType::CertificateHolderReference as u16,
            "Certificate Holder Reference",
        )?;
        let certificate_holder_reference = fixed_value::<CHR_SIZE>(&chr, "Certificate Holder Reference")?;

        let effective_date =
            expect_tlv(&mut body_value, CertificateContentType::CertificateEffectiveDate as u16, "Certificate Effective Date")?;
        let certificate_effective_date =
            TimeReal::new(u32::from_be_bytes(fixed_value::<4>(&effective_date, "Certificate Effective Date")?));

        let expiration_date =
            expect_tlv(&mut body_value, CertificateContentType::CertificateExpirationDate as u16, "Certificate Expiration Date")?;
        let certificate_expiration_date =
            TimeReal::new(u32::from_be_bytes(fixed_value::<4>(&expiration_date, "Certificate Expiration Date")?));

        if !body_value.is_empty() {
            return Err(Error::VerifyError("Unexpected fields in the Gen2 ECC Certificate Body.".to_string()));
        }

        Ok(Self {
            certificate_authority_reference,
            certificate_holder_authorisation,
            domain_parameters,
            public_point,
            certificate_holder_reference,
            certificate_effective_date,
            certificate_expiration_date,
            certificate_body: certificate_body.encoded.to_vec(),
            certificate_signature: fixed_value::<ECDSA_P256_SIGNATURE_SIZE>(&certificate_signature, "ECC Certificate Signature")?,
        })
    }

    pub fn from_bytes(data: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<Self> {
        Self::from_slice(data)
    }

    fn parse_public_key(data: &[u8]) -> Result<(String, [u8; SEC1_UNCOMPRESSED_P256_POINT_SIZE])> {
        let mut public_key_value = data;
        let domain_parameters =
            expect_tlv(&mut public_key_value, CertificateContentType::DomainParameters as u16, "Domain Parameters")?;
        let public_point = expect_tlv(&mut public_key_value, CertificateContentType::PublicPoint as u16, "Public Point")?;
        if !public_key_value.is_empty() {
            return Err(Error::VerifyError("Unexpected fields in the Gen2 public key.".to_string()));
        }

        Ok((
            oid_from_der(domain_parameters.value)?,
            fixed_value::<SEC1_UNCOMPRESSED_P256_POINT_SIZE>(&public_point, "Public Point")?,
        ))
    }
}

impl Readable<CardVerifiableCertificate> for CardVerifiableCertificate {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CardVerifiableCertificate> {
        let start_pos = reader.pos()?;
        let first_tag_byte = reader.read_u8()?;
        let (tag, tag_len) = if first_tag_byte & 0x1F == 0x1F {
            let second_tag_byte = reader.read_u8()?;
            if second_tag_byte & 0x80 != 0 {
                return Err(Error::VerifyError("Unsupported CVC tag with more than two bytes.".to_string()));
            }
            (((first_tag_byte as u16) << 8) | second_tag_byte as u16, 2)
        } else {
            (first_tag_byte as u16, 1)
        };

        if tag != CertificateContentType::ECCCertificate as u16 {
            return Err(Error::VerifyError("Gen2 certificate does not start with an ECC Certificate tag.".to_string()));
        }

        let first_length_byte = reader.read_u8()?;
        let (value_len, length_len) = if first_length_byte & 0x80 == 0 {
            (first_length_byte as usize, 1)
        } else {
            let length_byte_count = (first_length_byte & 0x7F) as usize;
            if length_byte_count == 0 {
                return Err(Error::VerifyError("Indefinite CVC lengths are not supported.".to_string()));
            }
            if length_byte_count > core::mem::size_of::<usize>() {
                return Err(Error::VerifyError("CVC length is too large.".to_string()));
            }
            let length_bytes = reader.read_into_vec(length_byte_count as u32)?;
            if length_bytes.first() == Some(&0) {
                return Err(Error::VerifyError("CVC length is not minimally encoded.".to_string()));
            }
            let value_len = length_bytes.iter().try_fold(0usize, |acc, &byte| {
                acc.checked_shl(8)
                    .and_then(|value| value.checked_add(byte as usize))
                    .ok_or_else(|| Error::VerifyError("CVC length overflows usize.".to_string()))
            })?;
            if value_len < 128 {
                return Err(Error::VerifyError("CVC long-form length is not minimally encoded.".to_string()));
            }
            (value_len, 1 + length_byte_count)
        };

        let header_len = tag_len + length_len;
        let encoded_len =
            header_len.checked_add(value_len).ok_or_else(|| Error::VerifyError("CVC tag length overflows usize.".to_string()))?;

        reader.seek(start_pos)?;
        let full_cert_bytes = reader.read_into_vec(encoded_len as u32)?;

        CardVerifiableCertificate::from_slice(&full_cert_bytes)
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    const ROOT_CERTIFICATE_HEX: &str = "7f2181c97f4e81825f2901004208fd45432001ffff015f4c07ff534d5244540d7f494e06092b240303020801010786410408c04e3926c8de85544240cde40dab70d2b47e0f83762522d7b0b8543b9b29dc80e5c67b82a62d55e3483ab4b00a24c2a2566c3786797a1a052822ab4bf1f2925f2008fd45432001ffff015f25045b21b0005f24049b8fae805f374065c62ac13ded147fa8d1d11a8f5bf2cf9e95db1b43d253b48b615b2fe70b3fd82aa8d33d27f0f4d7367c04903bbbe6375b643a19c5b83d19fc7485db476c7067";

    fn from_hex(hex: &str) -> Vec<u8> {
        assert_eq!(hex.len() % 2, 0);
        hex.as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| {
                let value = core::str::from_utf8(pair).unwrap();
                u8::from_str_radix(value, 16).unwrap()
            })
            .collect()
    }

    #[test]
    fn parses_card_verifiable_certificate_via_readable() {
        let cert_bytes = from_hex(ROOT_CERTIFICATE_HEX);
        let mut reader = BinMemoryBuffer::from(cert_bytes.clone());
        let cert = CardVerifiableCertificate::read(&mut reader).expect("readable must succeed");

        assert_eq!(cert.domain_parameters, BRAINPOOL_P256_R1_OID);
        assert_eq!(cert.certificate_holder_reference, [0xFD, 0x45, 0x43, 0x20, 0x01, 0xFF, 0xFF, 0x01]);
        assert_eq!(cert.certificate_authority_reference, [0xFD, 0x45, 0x43, 0x20, 0x01, 0xFF, 0xFF, 0x01]);
        assert_eq!(cert.certificate_signature.len(), ECDSA_P256_SIGNATURE_SIZE);
        assert_eq!(reader.pos().unwrap(), cert_bytes.len());
    }
}
