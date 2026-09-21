use bp256::BrainpoolP256r1;
use bp256::r1::ecdsa::Signature;
use ecdsa::VerifyingKey;
use sha2::{Digest, Sha256};
use signature::hazmat::PrehashVerifier;

use crate::tacho::{
    CardFileData, CardFileID, CardFilesMap, CertificateContentType, EquipmentType, TimeReal, VUFilesList,
    VUTransferResponseParameterID, VUVerifyItem, VUVerifyResult, VerifyItem, VerifyResult, VerifyResultStatus, VerifyStatus,
};
use crate::{Error, Result};

const GEN2_CERTIFICATE_SIZE: usize = 205;
const CAR_SIZE: usize = 8;
const CHR_SIZE: usize = 8;
const CHA_SIZE: usize = 7;
const CPI_VERSION_1: u8 = 0x00;
const ECDSA_P256_SIGNATURE_SIZE: usize = 64;
const SEC1_UNCOMPRESSED_P256_POINT_SIZE: usize = 65;
const BRAINPOOL_P256_R1_OID: &str = "1.3.36.3.3.2.8.1.1.7";

#[derive(Debug)]
struct Tlv<'a> {
    tag: u16,
    encoded: &'a [u8],
    value: &'a [u8],
}

fn parse_tlv(input: &[u8]) -> Result<Tlv<'_>> {
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

    Ok(Tlv { tag, encoded, value: &encoded[header_len..] })
}

fn take_tlv<'a>(input: &mut &'a [u8]) -> Result<Tlv<'a>> {
    let tlv = parse_tlv(input)?;
    *input = &input[tlv.encoded.len()..];
    Ok(tlv)
}

fn expect_tlv<'a>(input: &mut &'a [u8], expected_tag: u16, field: &str) -> Result<Tlv<'a>> {
    let tlv = take_tlv(input)?;
    if tlv.tag != expected_tag {
        return Err(Error::VerifyError(format!("Expected {field} tag {expected_tag:#06X}, found {:#06X}.", tlv.tag)));
    }
    Ok(tlv)
}

fn fixed_value<const N: usize>(tlv: &Tlv<'_>, field: &str) -> Result<[u8; N]> {
    tlv.value
        .try_into()
        .map_err(|_| Error::VerifyError(format!("Invalid {field} size: expected {N} bytes, found {} bytes.", tlv.value.len())))
}

fn oid_from_der(bytes: &[u8]) -> Result<String> {
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
        let mut component = 0u64;
        let mut complete = false;
        while index < bytes.len() {
            let byte = bytes[index];
            index += 1;
            component = component
                .checked_shl(7)
                .and_then(|value| value.checked_add((byte & 0x7F) as u64))
                .ok_or_else(|| Error::VerifyError("Domain parameters OID component overflows u64.".to_string()))?;
            if byte & 0x80 == 0 {
                complete = true;
                break;
            }
        }
        if !complete {
            return Err(Error::VerifyError("Truncated domain parameters OID.".to_string()));
        }
        parts.push(component.to_string());
    }

    Ok(parts.join("."))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Curve {
    BrainpoolP256r1,
}

impl Curve {
    fn from_oid(oid: &str) -> Result<Self> {
        match oid {
            BRAINPOOL_P256_R1_OID => Ok(Self::BrainpoolP256r1),
            _ => Err(Error::VerifyError(format!("Unsupported Gen2 ECDSA curve OID: {oid}."))),
        }
    }
}

#[derive(Debug, Clone)]
struct EcdsaPublicKey {
    curve: Curve,
    public_point: [u8; SEC1_UNCOMPRESSED_P256_POINT_SIZE],
}

impl EcdsaPublicKey {
    fn new(domain_parameters: &str, public_point: &[u8]) -> Result<Self> {
        let curve = Curve::from_oid(domain_parameters)?;
        let public_point: [u8; SEC1_UNCOMPRESSED_P256_POINT_SIZE] = public_point.try_into().map_err(|_| {
            Error::VerifyError(format!(
                "Invalid {domain_parameters} public point size: expected {SEC1_UNCOMPRESSED_P256_POINT_SIZE} bytes, found {} bytes.",
                public_point.len()
            ))
        })?;
        if public_point[0] != 0x04 {
            return Err(Error::VerifyError("Gen2 ECDSA public point must use uncompressed SEC1 encoding.".to_string()));
        }
        Ok(Self { curve, public_point })
    }

    fn verify(&self, prehash: &[u8], signature: &[u8]) -> Result<()> {
        match self.curve {
            Curve::BrainpoolP256r1 => {
                if signature.len() != ECDSA_P256_SIGNATURE_SIZE {
                    return Err(Error::VerifyError(format!(
                        "Invalid brainpoolP256r1 ECDSA signature size: expected {ECDSA_P256_SIGNATURE_SIZE} bytes, found {} bytes.",
                        signature.len()
                    )));
                }
                let verifying_key = VerifyingKey::<BrainpoolP256r1>::from_sec1_bytes(&self.public_point)
                    .map_err(|_| Error::VerifyError("Invalid brainpoolP256r1 public point.".to_string()))?;
                let signature = Signature::from_slice(signature)
                    .map_err(|_| Error::VerifyError("Invalid raw brainpoolP256r1 ECDSA signature.".to_string()))?;
                verifying_key
                    .verify_prehash(prehash, &signature)
                    .map_err(|_| Error::VerifyError("ECDSA signature verification failed.".to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Certificate {
    certificate_authority_reference: [u8; CAR_SIZE],
    certificate_holder_authorisation: [u8; CHA_SIZE],
    domain_parameters: String,
    public_point: [u8; SEC1_UNCOMPRESSED_P256_POINT_SIZE],
    certificate_holder_reference: [u8; CHR_SIZE],
    certificate_effective_date: TimeReal,
    certificate_expiration_date: TimeReal,
    certificate_body: Vec<u8>,
    certificate_signature: [u8; ECDSA_P256_SIGNATURE_SIZE],
}

impl Certificate {
    fn from_bytes(data: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<Self> {
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

#[derive(Debug)]
struct ERCACertificate {
    holder_reference: [u8; CHR_SIZE],
    ecdsa_public_key: EcdsaPublicKey,
}

impl ERCACertificate {
    fn new_at(data: &[u8; GEN2_CERTIFICATE_SIZE], validation_time: u32) -> Result<Self> {
        let certificate = Certificate::from_bytes(data)?;
        validate_certificate_validity(&certificate, validation_time)?;
        validate_certificate_role(&certificate, EquipmentType::EuropeanRootCA, "ERCA")?;
        if certificate.certificate_authority_reference != certificate.certificate_holder_reference {
            return Err(Error::VerifyError("ERCA CAR and CHR are not the same.".to_string()));
        }
        let ecdsa_public_key = EcdsaPublicKey::new(&certificate.domain_parameters, &certificate.public_point)?;
        ecdsa_public_key.verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature)?;
        Ok(Self { holder_reference: certificate.certificate_holder_reference, ecdsa_public_key })
    }
}

#[derive(Debug)]
struct VerifiedCertificate {
    end_of_validity: TimeReal,
    holder_reference: [u8; CHR_SIZE],
    ecdsa_public_key: EcdsaPublicKey,
}

impl VerifiedCertificate {
    fn new(certificate: &Certificate) -> Result<Self> {
        Ok(Self {
            end_of_validity: certificate.certificate_expiration_date.clone(),
            holder_reference: certificate.certificate_holder_reference,
            ecdsa_public_key: EcdsaPublicKey::new(&certificate.domain_parameters, &certificate.public_point)?,
        })
    }
}

fn current_unix_timestamp() -> Result<u32> {
    u32::try_from(time::OffsetDateTime::now_utc().unix_timestamp())
        .map_err(|_| Error::VerifyError("Current system time is outside the supported tachograph range.".to_string()))
}

fn validate_certificate_validity(certificate: &Certificate, validation_time: u32) -> Result<()> {
    let effective = certificate.certificate_effective_date.get_data();
    let expiration = certificate.certificate_expiration_date.get_data();
    if effective > expiration {
        return Err(Error::VerifyError("Certificate effective date is after its expiration date.".to_string()));
    }
    if validation_time < effective || validation_time > expiration {
        return Err(Error::VerifyError("Certificate is not valid at the verification time.".to_string()));
    }
    Ok(())
}

fn validate_certificate_role(certificate: &Certificate, expected: EquipmentType, name: &str) -> Result<()> {
    let actual = certificate.certificate_holder_authorisation[CHA_SIZE - 1];
    let expected_description = format!("{expected:?}");
    if actual != expected as u8 {
        return Err(Error::VerifyError(format!(
            "{name} certificate has equipment type {actual:#04X}; expected {expected_description}."
        )));
    }
    Ok(())
}

fn validate_card_sign_role(certificate: &Certificate) -> Result<()> {
    let equipment_type = certificate.certificate_holder_authorisation[CHA_SIZE - 1];
    if equipment_type != EquipmentType::DriverCardSign as u8 && equipment_type != EquipmentType::WorkshopCardSign as u8 {
        return Err(Error::VerifyError(format!(
            "Card signing certificate has unsupported equipment type {equipment_type:#04X}."
        )));
    }
    Ok(())
}

fn create_certificate_from(card_file_data: &CardFileData) -> Result<Certificate> {
    let data = card_file_data.data.as_ref().ok_or_else(|| Error::VerifyError("Missing certificate data.".to_string()))?;
    let certificate: &[u8; GEN2_CERTIFICATE_SIZE] = data
        .as_slice()
        .try_into()
        .map_err(|_| Error::VerifyError(format!("Invalid Gen2 certificate length: expected {GEN2_CERTIFICATE_SIZE} bytes.")))?;
    Certificate::from_bytes(certificate)
}

fn verify_ca_certificate_at(
    certificate: &Certificate,
    erca_certificate: &ERCACertificate,
    validation_time: u32,
) -> Result<VerifiedCertificate> {
    if certificate.certificate_authority_reference != erca_certificate.holder_reference {
        return Err(Error::VerifyError("MSCA CAR and ERCA CHR are not the same.".to_string()));
    }
    validate_certificate_role(certificate, EquipmentType::MemberStateCA, "MSCA")?;
    validate_certificate_validity(certificate, validation_time)?;
    erca_certificate
        .ecdsa_public_key
        .verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature)
        .map_err(|e| Error::VerifyError(format!("MSCA cert signature verification by ERCA failed: {e}")))?;
    VerifiedCertificate::new(certificate)
}

fn verify_link_certificate_at(
    certificate: &Certificate,
    erca_certificate: &ERCACertificate,
    validation_time: u32,
) -> Result<VerifiedCertificate> {
    if certificate.certificate_authority_reference != erca_certificate.holder_reference {
        return Err(Error::VerifyError("Link Certificate CAR and ERCA CHR are not the same.".to_string()));
    }
    validate_certificate_role(certificate, EquipmentType::EuropeanRootCA, "Link")?;
    validate_certificate_validity(certificate, validation_time)?;
    erca_certificate
        .ecdsa_public_key
        .verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature)?;
    VerifiedCertificate::new(certificate)
}

fn verify_ca_certificate_with_link_at(
    certificate: &Certificate,
    erca_certificate: &ERCACertificate,
    link_certificate: Option<&Certificate>,
    validation_time: u32,
) -> Result<(VerifiedCertificate, Option<VerifiedCertificate>)> {
    if certificate.certificate_authority_reference == erca_certificate.holder_reference {
        let msca_verified = verify_ca_certificate_at(certificate, erca_certificate, validation_time)?;
        Ok((msca_verified, None))
    } else if let Some(link_cert) = link_certificate {
        let link_verified = verify_link_certificate_at(link_cert, erca_certificate, validation_time)?;
        if certificate.certificate_authority_reference != link_verified.holder_reference {
            return Err(Error::VerifyError("MSCA CAR and Link Certificate CHR are not the same.".to_string()));
        }
        validate_certificate_role(certificate, EquipmentType::MemberStateCA, "MSCA")?;
        validate_certificate_validity(certificate, validation_time)?;
        link_verified
            .ecdsa_public_key
            .verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature)?;
        let msca_verified = VerifiedCertificate::new(certificate)?;
        Ok((msca_verified, Some(link_verified)))
    } else {
        Err(Error::VerifyError("MSCA CAR and ERCA CHR are not the same.".to_string()))
    }
}

fn verify_card_certificate_at(
    certificate: &Certificate,
    ca_certificate: &VerifiedCertificate,
    validation_time: u32,
) -> Result<VerifiedCertificate> {
    if certificate.certificate_authority_reference != ca_certificate.holder_reference {
        return Err(Error::VerifyError("Card_Sign CAR and MSCA CHR are not the same.".to_string()));
    }
    validate_card_sign_role(certificate)?;
    validate_certificate_validity(certificate, validation_time)?;
    ca_certificate.ecdsa_public_key.verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature)?;
    VerifiedCertificate::new(certificate)
}

/// These elementary files are present on a signed Gen2 card, but are not
/// themselves protected by the card-signing key.
fn is_non_signed_file(id: &CardFileID) -> bool {
    matches!(
        id,
        CardFileID::IC
            | CardFileID::ICC
            | CardFileID::CACertificate
            | CardFileID::CardCertificate
            | CardFileID::CardSignCertificate
            | CardFileID::LinkCertificate
            | CardFileID::CardDownload
    )
}

fn verify_data(data_files: &CardFilesMap, card_certificate: &VerifiedCertificate) -> Result<Vec<VerifyItem>> {
    let mut result = Vec::new();
    for (id, data_file) in data_files {
        if is_non_signed_file(id) {
            continue;
        }
        let Some(raw_data) = data_file.data.as_ref() else {
            result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::NotHaveData, end_of_validity: None });
            continue;
        };
        let Some(signature) = data_file.signature.as_ref() else {
            result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::NotHaveSignature, end_of_validity: None });
            continue;
        };
        if signature.len() != ECDSA_P256_SIGNATURE_SIZE {
            result.push(VerifyItem {
                card_file_id: id.clone(),
                status: VerifyStatus::InvalidSignatureSize,
                end_of_validity: None,
            });
            continue;
        }

        let status = if card_certificate.ecdsa_public_key.verify(&Sha256::digest(raw_data), signature).is_ok() {
            VerifyStatus::Valid
        } else {
            VerifyStatus::Invalid
        };
        result.push(VerifyItem { card_file_id: id.clone(), status, end_of_validity: None });
    }
    Ok(result)
}

fn result_status(items: &[VerifyItem]) -> VerifyResultStatus {
    if items.iter().all(|item| matches!(item.status, VerifyStatus::Valid)) {
        VerifyResultStatus::Valid
    } else if items.iter().any(|item| matches!(item.status, VerifyStatus::Valid)) {
        VerifyResultStatus::PartiallyValid
    } else {
        VerifyResultStatus::Invalid
    }
}

pub fn verify(data_files: &CardFilesMap, erca_pk: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<VerifyResult> {
    verify_with_time(data_files, erca_pk, None)
}

pub fn verify_with_time(
    data_files: &CardFilesMap,
    erca_pk: &[u8; GEN2_CERTIFICATE_SIZE],
    validation_time: Option<u32>,
) -> Result<VerifyResult> {
    if !data_files.contains_key(&CardFileID::IC) || !data_files.contains_key(&CardFileID::ICC) {
        return Ok(VerifyResult { status: VerifyResultStatus::Unsigned, result: Vec::new() });
    }

    let msca_cert_file = data_files
        .get(&CardFileID::CACertificate)
        .ok_or_else(|| Error::VerifyError("Missing MSCA Certificate (CACertificate).".to_string()))?;
    let (card_certificate_id, card_certificate_file) = data_files
        .get_key_value(&CardFileID::CardSignCertificate)
        .or_else(|| data_files.get_key_value(&CardFileID::CardCertificate))
        .ok_or_else(|| Error::VerifyError("Missing Card Sign Certificate.".to_string()))?;

    let validation_time = match validation_time {
        Some(time) => time,
        None => {
            if let Some(card_download_file) = data_files.get(&CardFileID::CardDownload) {
                if let Some(data) = &card_download_file.data {
                    if data.len() >= 4 { u32::from_be_bytes(data[..4].try_into().unwrap()) } else { current_unix_timestamp()? }
                } else {
                    current_unix_timestamp()?
                }
            } else {
                current_unix_timestamp()?
            }
        }
    };

    let erca_certificate = ERCACertificate::new_at(erca_pk, validation_time)?;
    let msca_certificate = create_certificate_from(msca_cert_file)?;
    let card_sign_certificate = create_certificate_from(card_certificate_file)?;
    let link_certificate = data_files.get(&CardFileID::LinkCertificate).map(create_certificate_from).transpose()?;

    let (msca_verified, link_verified) =
        verify_ca_certificate_with_link_at(&msca_certificate, &erca_certificate, link_certificate.as_ref(), validation_time)?;
    let card_verified = verify_card_certificate_at(&card_sign_certificate, &msca_verified, validation_time)?;

    let mut result = Vec::new();
    if let Some(link_verified) = link_verified {
        result.push(VerifyItem {
            card_file_id: CardFileID::LinkCertificate,
            status: VerifyStatus::Valid,
            end_of_validity: Some(link_verified.end_of_validity),
        });
    }
    result.push(VerifyItem {
        card_file_id: CardFileID::CACertificate,
        status: VerifyStatus::Valid,
        end_of_validity: Some(msca_verified.end_of_validity.clone()),
    });
    result.push(VerifyItem {
        card_file_id: card_certificate_id.clone(),
        status: VerifyStatus::Valid,
        end_of_validity: Some(card_verified.end_of_validity.clone()),
    });
    result.extend(verify_data(data_files, &card_verified)?);

    Ok(VerifyResult { status: result_status(&result), result })
}

fn vu_result_status(items: &[VUVerifyItem]) -> VerifyResultStatus {
    if items.is_empty() {
        VerifyResultStatus::Unsigned
    } else if items.iter().all(|item| matches!(item.status, VerifyStatus::Valid)) {
        VerifyResultStatus::Valid
    } else if items.iter().any(|item| matches!(item.status, VerifyStatus::Valid)) {
        VerifyResultStatus::PartiallyValid
    } else {
        VerifyResultStatus::Invalid
    }
}

fn is_overview_trep(trep_id: &VUTransferResponseParameterID) -> bool {
    matches!(
        trep_id,
        VUTransferResponseParameterID::Overview
            | VUTransferResponseParameterID::Gen2Overview
            | VUTransferResponseParameterID::Gen2v2Overview
    )
}

fn parse_certificate_record_array(raw_bytes: &[u8], offset: usize) -> Result<(usize, usize, &[u8])> {
    let header = raw_bytes
        .get(offset..offset + 5)
        .ok_or_else(|| Error::VerifyError("Overview raw data too short for record array header.".to_string()))?;
    let record_size = u16::from_be_bytes(header[1..3].try_into().unwrap()) as usize;
    let no_of_records = u16::from_be_bytes(header[3..5].try_into().unwrap()) as usize;
    if record_size != GEN2_CERTIFICATE_SIZE || no_of_records == 0 {
        return Err(Error::VerifyError("Invalid certificate record array in Overview.".to_string()));
    }
    let data_len = record_size * no_of_records;
    let data = raw_bytes
        .get(offset + 5..offset + 5 + data_len)
        .ok_or_else(|| Error::VerifyError("Truncated certificate data in Overview.".to_string()))?;
    Ok((record_size, no_of_records, data))
}

fn extract_vu_certificates(raw_bytes: &[u8]) -> Result<(Certificate, Option<Certificate>, Certificate)> {
    let (msca_size, msca_count, msca_data) = parse_certificate_record_array(raw_bytes, 0)?;
    let vu_offset = 5 + (msca_size * msca_count);
    let (_vu_size, _vu_count, vu_data) = parse_certificate_record_array(raw_bytes, vu_offset)?;

    let vu_cert_bytes: &[u8; GEN2_CERTIFICATE_SIZE] = vu_data
        .get(..GEN2_CERTIFICATE_SIZE)
        .ok_or_else(|| Error::VerifyError("Truncated VU certificate.".to_string()))?
        .try_into()
        .unwrap();
    let vu_sign_cert = Certificate::from_bytes(vu_cert_bytes)?;

    let (msca_cert, link_cert) = if msca_count > 1 {
        let cert0_bytes: &[u8; GEN2_CERTIFICATE_SIZE] = msca_data
            .get(..GEN2_CERTIFICATE_SIZE)
            .ok_or_else(|| Error::VerifyError("Truncated certificate in Overview.".to_string()))?
            .try_into()
            .unwrap();
        let cert1_bytes: &[u8; GEN2_CERTIFICATE_SIZE] = msca_data
            .get(GEN2_CERTIFICATE_SIZE..2 * GEN2_CERTIFICATE_SIZE)
            .ok_or_else(|| Error::VerifyError("Truncated certificate in Overview.".to_string()))?
            .try_into()
            .unwrap();
        let cert0 = Certificate::from_bytes(cert0_bytes)?;
        let cert1 = Certificate::from_bytes(cert1_bytes)?;
        if cert0.certificate_holder_authorisation[CHA_SIZE - 1] == EquipmentType::EuropeanRootCA as u8 {
            (cert1, Some(cert0))
        } else {
            (cert0, Some(cert1))
        }
    } else {
        let msca_cert_bytes: &[u8; GEN2_CERTIFICATE_SIZE] = msca_data
            .get(..GEN2_CERTIFICATE_SIZE)
            .ok_or_else(|| Error::VerifyError("Truncated MSCA certificate in Overview.".to_string()))?
            .try_into()
            .unwrap();
        (Certificate::from_bytes(msca_cert_bytes)?, None)
    };

    Ok((msca_cert, link_cert, vu_sign_cert))
}

fn verify_vu_certificates_at(
    raw_bytes: &[u8],
    erca_pk: &[u8; GEN2_CERTIFICATE_SIZE],
    validation_time: u32,
) -> Result<VerifiedCertificate> {
    let (msca_certificate, link_certificate, vu_sign_certificate) = extract_vu_certificates(raw_bytes)?;
    let erca_certificate = ERCACertificate::new_at(erca_pk, validation_time)?;
    let (msca_verified, _link_verified) =
        verify_ca_certificate_with_link_at(&msca_certificate, &erca_certificate, link_certificate.as_ref(), validation_time)?;

    if vu_sign_certificate.certificate_authority_reference != msca_verified.holder_reference {
        return Err(Error::VerifyError("VU_Sign CAR and MSCA CHR are not the same.".to_string()));
    }
    validate_certificate_role(&vu_sign_certificate, EquipmentType::VehicleUnitSign, "VU Sign")?;
    validate_certificate_validity(&vu_sign_certificate, validation_time)?;
    msca_verified
        .ecdsa_public_key
        .verify(&Sha256::digest(&vu_sign_certificate.certificate_body), &vu_sign_certificate.certificate_signature)
        .map_err(|e| Error::VerifyError(format!("VU cert signature verification by MSCA failed: {e}")))?;
    VerifiedCertificate::new(&vu_sign_certificate)
}

fn verify_vu_data(data_files: &VUFilesList, vu_verified: &VerifiedCertificate) -> Vec<VUVerifyItem> {
    let mut result = Vec::new();
    for file in data_files {
        let Some(raw_data) = file.data.as_ref() else {
            result.push(VUVerifyItem {
                trep_id: file.trep_id.clone(),
                position: file.position,
                status: VerifyStatus::NotHaveData,
                end_of_validity: None,
            });
            continue;
        };
        let Some(signature) = file.signature.as_ref() else {
            result.push(VUVerifyItem {
                trep_id: file.trep_id.clone(),
                position: file.position,
                status: VerifyStatus::NotHaveSignature,
                end_of_validity: None,
            });
            continue;
        };
        if signature.len() != ECDSA_P256_SIGNATURE_SIZE {
            result.push(VUVerifyItem {
                trep_id: file.trep_id.clone(),
                position: file.position,
                status: VerifyStatus::InvalidSignatureSize,
                end_of_validity: None,
            });
            continue;
        }

        let status = if vu_verified.ecdsa_public_key.verify(&Sha256::digest(raw_data), signature).is_ok() {
            VerifyStatus::Valid
        } else {
            VerifyStatus::Invalid
        };

        let end_of_validity = if is_overview_trep(&file.trep_id) { Some(vu_verified.end_of_validity.clone()) } else { None };

        result.push(VUVerifyItem { trep_id: file.trep_id.clone(), position: file.position, status, end_of_validity });
    }
    result
}

pub fn verify_vu(data_files: &VUFilesList, erca_pk: &[u8; GEN2_CERTIFICATE_SIZE]) -> Result<VUVerifyResult> {
    verify_vu_with_time(data_files, erca_pk, None)
}

pub fn verify_vu_with_time(
    data_files: &VUFilesList,
    erca_pk: &[u8; GEN2_CERTIFICATE_SIZE],
    validation_time: Option<u32>,
) -> Result<VUVerifyResult> {
    let overview = data_files
        .iter()
        .find(|f| is_overview_trep(&f.trep_id))
        .ok_or_else(|| Error::VerifyError("Missing Overview TREP in VU data files.".to_string()))?;

    let raw_bytes =
        overview.raw_data.as_ref().ok_or_else(|| Error::VerifyError("Missing raw data for Overview TREP.".to_string()))?;

    let validation_time = match validation_time {
        Some(t) => t,
        None => current_unix_timestamp()?,
    };

    let vu_verified = verify_vu_certificates_at(raw_bytes, erca_pk, validation_time)?;
    let result = verify_vu_data(data_files, &vu_verified);

    Ok(VUVerifyResult { status: vu_result_status(&result), result })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ecdsa::SigningKey;
    use signature::hazmat::PrehashSigner;

    use super::*;
    use crate::tacho::VUFileData;

    const ROOT_CERTIFICATE_HEX: &str = "7f2181c97f4e81825f2901004208fd45432001ffff015f4c07ff534d5244540d7f494e06092b240303020801010786410408c04e3926c8de85544240cde40dab70d2b47e0f83762522d7b0b8543b9b29dc80e5c67b82a62d55e3483ab4b00a24c2a2566c3786797a1a052822ab4bf1f2925f2008fd45432001ffff015f25045b21b0005f24049b8fae805f374065c62ac13ded147fa8d1d11a8f5bf2cf9e95db1b43d253b48b615b2fe70b3fd82aa8d33d27f0f4d7367c04903bbbe6375b643a19c5b83d19fc7485db476c7067";
    const MSCA_CERTIFICATE_HEX: &str = "7f2181c97f4e81825f2901004208fd45432001ffff015f4c07ff534d5244540e7f494e06092b24030302080101078641044cdeb93fb90256c7a7ebf9df3214560b6d2f4f2e72f3bb1544fcd8061ce1653e37f60de125bea0fcd5076f94557b605b40ba1c0a0a3e96de9cb37c4c053a6f095f20081948522003ff02015f2504633588c05f2404708821405f37401139ba4ee2b4b4180e56f65c90f1aedc876804ab54fb97abe9a7a4e7bcb1ea2d89e6a446e3ad4e6f82676530980b872f6885ad4559a69e08b8fce023a201b727";

    fn from_hex(hex: &str) -> Vec<u8> {
        assert_eq!(hex.len() % 2, 0);
        hex.as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let value = core::str::from_utf8(pair).unwrap();
                u8::from_str_radix(value, 16).unwrap()
            })
            .collect()
    }

    fn certificate(hex: &str) -> [u8; GEN2_CERTIFICATE_SIZE] {
        from_hex(hex).try_into().unwrap()
    }

    fn signing_key(last_byte: u8) -> SigningKey<BrainpoolP256r1> {
        let mut scalar = [0u8; 32];
        scalar[31] = last_byte;
        SigningKey::from_slice(&scalar).unwrap()
    }

    fn sign(signer: &SigningKey<BrainpoolP256r1>, payload: &[u8]) -> Vec<u8> {
        let signature: Signature = signer.sign_prehash(&Sha256::digest(payload)).unwrap();
        signature.to_bytes().as_slice().to_vec()
    }

    fn certificate_for(
        holder_key: &SigningKey<BrainpoolP256r1>,
        issuer_key: &SigningKey<BrainpoolP256r1>,
        car: [u8; CAR_SIZE],
        chr: [u8; CHR_SIZE],
        equipment_type: u8,
    ) -> [u8; GEN2_CERTIFICATE_SIZE] {
        let public_point = holder_key.verifying_key().to_sec1_point(false);
        let public_point = public_point.as_bytes();
        assert_eq!(public_point.len(), SEC1_UNCOMPRESSED_P256_POINT_SIZE);

        let mut body = vec![0x7F, 0x4E, 0x81, 0x82];
        body.extend_from_slice(&[0x5F, 0x29, 0x01, CPI_VERSION_1]);
        body.extend_from_slice(&[0x42, 0x08]);
        body.extend_from_slice(&car);
        body.extend_from_slice(&[0x5F, 0x4C, 0x07, 0xFF, b'S', b'M', b'R', b'D', b'T', equipment_type]);
        body.extend_from_slice(&[0x7F, 0x49, 0x4E, 0x06, 0x09]);
        body.extend_from_slice(&[0x2B, 0x24, 0x03, 0x03, 0x02, 0x08, 0x01, 0x01, 0x07]);
        body.extend_from_slice(&[0x86, 0x41]);
        body.extend_from_slice(public_point);
        body.extend_from_slice(&[0x5F, 0x20, 0x08]);
        body.extend_from_slice(&chr);
        body.extend_from_slice(&[0x5F, 0x25, 0x04, 0x00, 0x00, 0x00, 0x00]);
        body.extend_from_slice(&[0x5F, 0x24, 0x04, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(body.len(), 134);

        let signature = sign(issuer_key, &body);
        let mut certificate = vec![0x7F, 0x21, 0x81, 0xC9];
        certificate.extend_from_slice(&body);
        certificate.extend_from_slice(&[0x5F, 0x37, 0x40]);
        certificate.extend_from_slice(&signature);
        certificate.try_into().unwrap()
    }

    fn data_file(card_file_id: CardFileID, data: Vec<u8>, signature: Option<Vec<u8>>) -> CardFileData {
        CardFileData {
            card_file_id,
            appendix: 2,
            card_file_notes: String::new(),
            size: data.len() as u32,
            signature,
            data: Some(data),
        }
    }

    #[test]
    fn parses_the_known_gen2_erca_certificate() {
        let certificate = Certificate::from_bytes(&certificate(ROOT_CERTIFICATE_HEX)).unwrap();

        assert_eq!(certificate.domain_parameters, BRAINPOOL_P256_R1_OID);
        assert_eq!(certificate.certificate_holder_reference, [0xFD, 0x45, 0x43, 0x20, 0x01, 0xFF, 0xFF, 0x01]);
        assert_eq!(certificate.certificate_signature.len(), ECDSA_P256_SIGNATURE_SIZE);
    }

    #[test]
    fn verifies_the_known_gen2_msca_chain() {
        let validation_time = 1_735_689_600; // 2025-01-01T00:00:00Z
        let erca = ERCACertificate::new_at(&certificate(ROOT_CERTIFICATE_HEX), validation_time).unwrap();
        let msca = Certificate::from_bytes(&certificate(MSCA_CERTIFICATE_HEX)).unwrap();

        let verified = verify_ca_certificate_at(&msca, &erca, validation_time).unwrap();

        assert_eq!(verified.holder_reference, [0x19, 0x48, 0x52, 0x20, 0x03, 0xFF, 0x02, 0x01]);
    }

    #[test]
    fn verifies_the_known_gen2_erca_self_signature() {
        let root = certificate(ROOT_CERTIFICATE_HEX);
        let certificate = Certificate::from_bytes(&root).unwrap();
        let erca = ERCACertificate::new_at(&root, 1_735_689_600).unwrap();

        erca.ecdsa_public_key.verify(&Sha256::digest(&certificate.certificate_body), &certificate.certificate_signature).unwrap();
    }

    #[test]
    fn rejects_a_tampered_erca_self_signature() {
        let mut root = certificate(ROOT_CERTIFICATE_HEX);
        root[GEN2_CERTIFICATE_SIZE - 1] ^= 0x01;

        assert!(ERCACertificate::new_at(&root, 1_735_689_600).is_err());
    }

    #[test]
    fn verifies_a_complete_fixed_size_gen2_card_chain_and_data() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let card_key = signing_key(3);
        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let card_chr = [0x30; CHR_SIZE];
        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let card_sign = certificate_for(&card_key, &msca_key, msca_chr, card_chr, EquipmentType::DriverCardSign as u8);

        let mut data_files = HashMap::new();
        data_files.insert(CardFileID::IC, data_file(CardFileID::IC, vec![0x01, 0x02], None));
        data_files.insert(CardFileID::ICC, data_file(CardFileID::ICC, vec![0x03, 0x04], None));
        data_files.insert(CardFileID::CardDownload, data_file(CardFileID::CardDownload, vec![0x05], None));
        let signed_data = b"Gen2 signed card data".to_vec();
        let signature = sign(&card_key, &signed_data);
        data_files.insert(CardFileID::EventsData, data_file(CardFileID::EventsData, signed_data, Some(signature)));
        data_files.insert(CardFileID::CACertificate, data_file(CardFileID::CACertificate, msca.to_vec(), None));
        data_files.insert(CardFileID::CardSignCertificate, data_file(CardFileID::CardSignCertificate, card_sign.to_vec(), None));

        let verified = verify(&data_files, &root).unwrap();
        assert!(matches!(verified.status, VerifyResultStatus::Valid));
        assert_eq!(verified.result.len(), 3);
        assert!(verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)));
        assert!(verified.result.iter().all(|item| !matches!(item.card_file_id, CardFileID::IC | CardFileID::ICC)));
        assert!(verified.result.iter().all(|item| item.card_file_id != CardFileID::CardDownload));

        data_files.get_mut(&CardFileID::EventsData).unwrap().data.as_mut().unwrap()[0] ^= 0x01;
        let tampered = verify(&data_files, &root).unwrap();
        assert!(matches!(tampered.status, VerifyResultStatus::PartiallyValid));
        assert!(
            tampered
                .result
                .iter()
                .any(|item| { item.card_file_id == CardFileID::EventsData && matches!(item.status, VerifyStatus::Invalid) })
        );
    }

    #[test]
    fn rejects_a_certificate_that_is_not_205_bytes() {
        let certificate = data_file(CardFileID::CACertificate, vec![0; GEN2_CERTIFICATE_SIZE - 1], None);

        let error = create_certificate_from(&certificate).unwrap_err();
        assert!(matches!(error, Error::VerifyError(message) if message.contains("expected 205 bytes")));
    }

    #[test]
    fn rejects_a_tampered_gen2_certificate_body() {
        let validation_time = 1_735_689_600;
        let erca = ERCACertificate::new_at(&certificate(ROOT_CERTIFICATE_HEX), validation_time).unwrap();
        let mut msca = certificate(MSCA_CERTIFICATE_HEX);
        msca[50] ^= 0x01;
        let msca = Certificate::from_bytes(&msca).unwrap();

        assert!(verify_ca_certificate_at(&msca, &erca, validation_time).is_err());
    }

    #[test]
    fn rejects_the_wrong_certificate_authority_reference_before_crypto() {
        let validation_time = 1_735_689_600;
        let erca = ERCACertificate::new_at(&certificate(ROOT_CERTIFICATE_HEX), validation_time).unwrap();
        let mut msca = certificate(MSCA_CERTIFICATE_HEX);
        msca[15] ^= 0x01;
        let msca = Certificate::from_bytes(&msca).unwrap();

        let error = verify_ca_certificate_at(&msca, &erca, validation_time).unwrap_err();
        assert!(matches!(error, Error::VerifyError(message) if message.contains("CAR and ERCA CHR")));
    }

    #[test]
    fn verifies_a_complete_fixed_size_gen2_card_chain_with_link_certificate() {
        let root_key_old = signing_key(1);
        let root_key_new = signing_key(2);
        let msca_key = signing_key(3);
        let card_key = signing_key(4);

        let root_old_chr = [0x10; CHR_SIZE];
        let root_new_chr = [0x20; CHR_SIZE];
        let msca_chr = [0x30; CHR_SIZE];
        let card_chr = [0x40; CHR_SIZE];

        let root_old =
            certificate_for(&root_key_old, &root_key_old, root_old_chr, root_old_chr, EquipmentType::EuropeanRootCA as u8);
        let link_cert =
            certificate_for(&root_key_new, &root_key_old, root_old_chr, root_new_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key_new, root_new_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let card_sign = certificate_for(&card_key, &msca_key, msca_chr, card_chr, EquipmentType::DriverCardSign as u8);

        let mut data_files = HashMap::new();
        data_files.insert(CardFileID::IC, data_file(CardFileID::IC, vec![0x01, 0x02], None));
        data_files.insert(CardFileID::ICC, data_file(CardFileID::ICC, vec![0x03, 0x04], None));
        let signed_data = b"Gen2 signed card data with link cert".to_vec();
        let signature = sign(&card_key, &signed_data);
        data_files.insert(CardFileID::EventsData, data_file(CardFileID::EventsData, signed_data, Some(signature)));
        data_files.insert(CardFileID::LinkCertificate, data_file(CardFileID::LinkCertificate, link_cert.to_vec(), None));
        data_files.insert(CardFileID::CACertificate, data_file(CardFileID::CACertificate, msca.to_vec(), None));
        data_files.insert(CardFileID::CardSignCertificate, data_file(CardFileID::CardSignCertificate, card_sign.to_vec(), None));

        let verified = verify_with_time(&data_files, &root_old, Some(1_735_689_600)).unwrap();
        assert!(matches!(verified.status, VerifyResultStatus::Valid));
        assert_eq!(verified.result.len(), 4);
        assert!(
            verified
                .result
                .iter()
                .any(|item| item.card_file_id == CardFileID::LinkCertificate && matches!(item.status, VerifyStatus::Valid))
        );
        assert!(
            verified
                .result
                .iter()
                .any(|item| item.card_file_id == CardFileID::CACertificate && matches!(item.status, VerifyStatus::Valid))
        );
        assert!(
            verified
                .result
                .iter()
                .any(|item| item.card_file_id == CardFileID::CardSignCertificate && matches!(item.status, VerifyStatus::Valid))
        );
        assert!(
            verified
                .result
                .iter()
                .any(|item| item.card_file_id == CardFileID::EventsData && matches!(item.status, VerifyStatus::Valid))
        );
    }

    #[test]
    fn verifies_a_complete_fixed_size_gen2_vu_chain_and_data() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let vu_key = signing_key(3);

        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let vu_chr = [0x30; CHR_SIZE];

        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let vu_sign = certificate_for(&vu_key, &msca_key, msca_chr, vu_chr, EquipmentType::VehicleUnitSign as u8);

        let overview_payload = b"Gen2 signed VU overview payload";
        let overview_sig = sign(&vu_key, overview_payload);

        let mut overview_raw = Vec::new();
        // MSCA record array: type=0, size=205, count=1
        overview_raw.extend_from_slice(&[0x00, 0x00, 0xCD, 0x00, 0x01]);
        overview_raw.extend_from_slice(&msca);
        // VU record array: type=0, size=205, count=1
        overview_raw.extend_from_slice(&[0x00, 0x00, 0xCD, 0x00, 0x01]);
        overview_raw.extend_from_slice(&vu_sign);
        // Payload
        overview_raw.extend_from_slice(overview_payload);
        // Signature record array: type=0, size=64, count=1
        overview_raw.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x01]);
        overview_raw.extend_from_slice(&overview_sig);

        let overview_file = VUFileData {
            trep_id: VUTransferResponseParameterID::Gen2Overview,
            position: 1,
            size: overview_payload.len() as u32,
            signature: Some(overview_sig.clone()),
            signatures: vec![overview_sig],
            data: Some(overview_payload.to_vec()),
            raw_data: Some(overview_raw),
        };

        let activity_payload = b"Gen2 signed VU activity payload";
        let activity_sig = sign(&vu_key, activity_payload);
        let activity_file = VUFileData {
            trep_id: VUTransferResponseParameterID::Gen2Activities,
            position: 2,
            size: activity_payload.len() as u32,
            signature: Some(activity_sig.clone()),
            signatures: vec![activity_sig],
            data: Some(activity_payload.to_vec()),
            raw_data: None,
        };

        let data_files = vec![overview_file, activity_file];
        let verified = verify_vu_with_time(&data_files, &root, Some(1_735_689_600)).unwrap();
        assert!(matches!(verified.status, VerifyResultStatus::Valid));
        assert_eq!(verified.result.len(), 2);
        assert!(verified.result.iter().all(|item| matches!(item.status, VerifyStatus::Valid)));
    }

    #[test]
    fn rejects_a_tampered_gen2_vu_signature() {
        let root_key = signing_key(1);
        let msca_key = signing_key(2);
        let vu_key = signing_key(3);

        let root_chr = [0x10; CHR_SIZE];
        let msca_chr = [0x20; CHR_SIZE];
        let vu_chr = [0x30; CHR_SIZE];

        let root = certificate_for(&root_key, &root_key, root_chr, root_chr, EquipmentType::EuropeanRootCA as u8);
        let msca = certificate_for(&msca_key, &root_key, root_chr, msca_chr, EquipmentType::MemberStateCA as u8);
        let vu_sign = certificate_for(&vu_key, &msca_key, msca_chr, vu_chr, EquipmentType::VehicleUnitSign as u8);

        let overview_payload = b"Gen2 signed VU overview payload";
        let overview_sig = sign(&vu_key, overview_payload);

        let mut overview_raw = Vec::new();
        overview_raw.extend_from_slice(&[0x00, 0x00, 0xCD, 0x00, 0x01]);
        overview_raw.extend_from_slice(&msca);
        overview_raw.extend_from_slice(&[0x00, 0x00, 0xCD, 0x00, 0x01]);
        overview_raw.extend_from_slice(&vu_sign);
        overview_raw.extend_from_slice(overview_payload);
        overview_raw.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x01]);
        overview_raw.extend_from_slice(&overview_sig);

        let overview_file = VUFileData {
            trep_id: VUTransferResponseParameterID::Gen2Overview,
            position: 1,
            size: overview_payload.len() as u32,
            signature: Some(overview_sig.clone()),
            signatures: vec![overview_sig],
            data: Some(overview_payload.to_vec()),
            raw_data: Some(overview_raw),
        };

        let mut tampered_sig = sign(&vu_key, b"Gen2 signed VU activity payload");
        tampered_sig[0] ^= 0x01;
        let activity_file = VUFileData {
            trep_id: VUTransferResponseParameterID::Gen2Activities,
            position: 2,
            size: 31,
            signature: Some(tampered_sig.clone()),
            signatures: vec![tampered_sig],
            data: Some(b"Gen2 signed VU activity payload".to_vec()),
            raw_data: None,
        };

        let data_files = vec![overview_file, activity_file];
        let verified = verify_vu_with_time(&data_files, &root, Some(1_735_689_600)).unwrap();
        assert!(matches!(verified.status, VerifyResultStatus::PartiallyValid));
        assert!(
            verified.result.iter().any(|item| item.trep_id == VUTransferResponseParameterID::Gen2Activities
                && matches!(item.status, VerifyStatus::Invalid))
        );
    }
}
