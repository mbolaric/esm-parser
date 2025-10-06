use binary_data::BinMemoryBuffer;
use log::debug;
use num_bigint::BigUint;
use sha1::{Digest, Sha1};

use crate::{
    Error, Readable, Result,
    helpers::get_sub_array,
    tacho::{CardFileData, CardFileID, CardFilesMap, TimeReal, VerifyItem, VerifyResult, VerifyResultStatus, VerifyStatus},
};

const SIG_SIZE: usize = 128;
const PKR_SIZE: usize = 58;
const CAR_SIZE: usize = 8;
const CR_SIZE: usize = 106;
const HASH_SIZE: usize = 20;
const RSA_KEY_SIZE: usize = 136;
const DECRYPTED_CERT_SIZE: usize = 164;

const DATA_PATTERN: [u8; 15] = [48, 33, 48, 9, 6, 5, 43, 14, 3, 2, 26, 5, 0, 4, 20];
const SIGNATURE_PADDING: [u8; 90] = [0xFF; 90];

#[derive(Debug)]
struct RsaPublicKey {
    modulus: BigUint,
    exponent: BigUint,
}

impl RsaPublicKey {
    fn new(data: [u8; RSA_KEY_SIZE]) -> Self {
        let modulus = BigUint::from_bytes_be(&data[..SIG_SIZE]);
        let exponent = BigUint::from_bytes_be(&data[SIG_SIZE..]);
        Self { modulus, exponent }
    }

    fn perform(&self, signature: &[u8; SIG_SIZE]) -> Vec<u8> {
        let base = BigUint::from_bytes_be(signature);
        base.modpow(&self.exponent, &self.modulus).to_bytes_be()
    }
}

#[derive(Debug)]
struct ECPKCertificate {
    holder_reference: [u8; CAR_SIZE],
    rsa_public_key: RsaPublicKey,
}

impl ECPKCertificate {
    fn new(data: &[u8; 144]) -> Result<Self> {
        let holder_reference = data[..CAR_SIZE]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get holder reference from ECPKCertificate".to_string()))?;
        let rsa_public_key_data: [u8; RSA_KEY_SIZE] = data[CAR_SIZE..]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get rsa public key from ECPKCertificate".to_string()))?;
        let rsa_public_key = RsaPublicKey::new(rsa_public_key_data);
        Ok(Self { holder_reference, rsa_public_key })
    }
}

#[derive(Debug)]
struct Certificate {
    signature: [u8; SIG_SIZE],
    public_key_remainder: [u8; PKR_SIZE],
    certification_authority_reference: [u8; CAR_SIZE],
}

impl Certificate {
    fn from_bytes(data: &[u8; 194]) -> Result<Self> {
        let signature = data[0..SIG_SIZE]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get signature from Certificate".to_string()))?;
        let public_key_remainder = data[SIG_SIZE..SIG_SIZE + PKR_SIZE]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get public key remainder from Certificate".to_string()))?;
        let certification_authority_reference = data[SIG_SIZE + PKR_SIZE..]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get certification authority reference from Certificate".to_string()))?;
        Ok(Self { signature, public_key_remainder, certification_authority_reference })
    }

    pub fn decrypt(&self, cr: &[u8], h: &[u8; HASH_SIZE]) -> Result<DecryptedCertificate> {
        let data = [cr, &self.public_key_remainder].concat();

        let mut hasher = Sha1::new();
        hasher.update(&data);
        let hash: [u8; HASH_SIZE] = hasher.finalize().into();

        if &hash != h {
            return Err(Error::VerifyError("Certificate Hash mismatch!".to_string()));
        }

        let decrypted_cert_data: [u8; DECRYPTED_CERT_SIZE] =
            data.try_into().map_err(|_| Error::VerifyError("Could not decrypt certificate".to_string()))?;

        DecryptedCertificate::new(decrypted_cert_data)
    }
}

#[derive(Debug)]
struct DecryptedCertificate {
    pub end_of_validity: TimeReal,
    pub holder_reference: [u8; CAR_SIZE],
    pub rsa_public_key: RsaPublicKey,
}

impl DecryptedCertificate {
    fn new(data: [u8; DECRYPTED_CERT_SIZE]) -> Result<Self> {
        let mut reader = BinMemoryBuffer::from(&data[16..20]);
        let end_of_validity = TimeReal::read(&mut reader)?;
        let holder_reference: [u8; CAR_SIZE] = data[20..28]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get holder reference from DecryptedCertificate".to_string()))?;
        let rsa_public_key_data: [u8; RSA_KEY_SIZE] = data[28..]
            .try_into()
            .map_err(|_| Error::VerifyError("Could not get rsa public key from DecryptedCertificate".to_string()))?;
        let rsa_public_key = RsaPublicKey::new(rsa_public_key_data);

        Ok(Self { end_of_validity, holder_reference, rsa_public_key })
    }
}

fn create_certificate_from(card_file_data: &CardFileData) -> Result<Certificate> {
    let signature = card_file_data.data.as_ref().ok_or_else(|| Error::VerifyError("Missing Certificate Data.".to_string()))?;

    let signature_array: &[u8; 194] = signature
        .as_slice()
        .try_into()
        .map_err(|_| Error::VerifyError("Invalid signature length in Certificate.".to_string()))?;

    Certificate::from_bytes(signature_array)
}

fn decrypt_ca_certificate(certificate: &Certificate, ec_pk_certificate: &ECPKCertificate) -> Result<DecryptedCertificate> {
    if certificate.certification_authority_reference != ec_pk_certificate.holder_reference {
        return Err(Error::VerifyError(
            "CA Certification authority reference and ERCA holder reference are not same".to_string(),
        ));
    }

    let perf_ret = ec_pk_certificate.rsa_public_key.perform(&certificate.signature);
    if perf_ret.first() != Some(&106) || perf_ret.last() != Some(&188) {
        return Err(Error::VerifyError(format!("CA RsaPublicKey need to start with {:2X} and end with {:2X}", 106, 188)));
    }

    let cr: [u8; CR_SIZE] =
        perf_ret[1..107].try_into().map_err(|_| Error::VerifyError("Could not get CR from RsaPublicKey".to_string()))?;
    let h: [u8; HASH_SIZE] =
        perf_ret[107..127].try_into().map_err(|_| Error::VerifyError("Could not get HASH from RsaPublicKey".to_string()))?;

    certificate.decrypt(&cr, &h)
}

fn decrypt_card_certificate(certificate: &Certificate, ca_certificate: &DecryptedCertificate) -> Result<DecryptedCertificate> {
    if certificate.certification_authority_reference != ca_certificate.holder_reference {
        return Err(Error::VerifyError(
            "Certification authority referenceCould and ERCA holder reference are not same".to_string(),
        ));
    }

    let perf_ret = ca_certificate.rsa_public_key.perform(&certificate.signature);
    if perf_ret.first() != Some(&106) || perf_ret.last() != Some(&188) {
        return Err(Error::VerifyError(format!("RsaPublicKey need to start with {:2X} and end with {:2X}", 106, 188)));
    }

    let cr: [u8; CR_SIZE] =
        perf_ret[1..107].try_into().map_err(|_| Error::VerifyError("Could not get CR from RsaPublicKey".to_string()))?;
    let h: [u8; HASH_SIZE] =
        perf_ret[107..127].try_into().map_err(|_| Error::VerifyError("Could not get HASH from RsaPublicKey".to_string()))?;

    certificate.decrypt(&cr, &h)
}

fn verify_data(data_files: &CardFilesMap, card_certificate: &DecryptedCertificate) -> Result<Vec<VerifyItem>> {
    let mut result: Vec<VerifyItem> = Vec::new();
    for data_file in data_files.iter() {
        let id = data_file.0;
        if id == &CardFileID::CACertificate || id == &CardFileID::CardCertificate {
            continue;
        }

        let data = data_file.1;

        if data.data.is_none() {
            result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::NotHaveData, end_of_validity: None });
            continue;
        }

        if data.signature.is_none() {
            result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::NotHaveSignature, end_of_validity: None });
            continue;
        }

        let raw_data = data.data.as_ref().unwrap();
        match data.signature.as_ref().unwrap()[0..SIG_SIZE].try_into() {
            Ok(signature) => {
                let perf_ret = card_certificate.rsa_public_key.perform(&signature);

                let mut hasher = Sha1::new();
                hasher.update(raw_data);
                let hash: [u8; HASH_SIZE] = hasher.finalize().into();

                if perf_ret.len() == 127
                    && hash.as_slice() == get_sub_array(&perf_ret, 107, 20)
                    && get_sub_array(&perf_ret, 92, 15) == DATA_PATTERN
                    && get_sub_array(&perf_ret, 1, 90) == SIGNATURE_PADDING
                {
                    result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::Valid, end_of_validity: None });
                    continue;
                }

                result.push(VerifyItem { card_file_id: id.clone(), status: VerifyStatus::Invalid, end_of_validity: None });
            }
            Err(_) => {
                result.push(VerifyItem {
                    card_file_id: id.clone(),
                    status: VerifyStatus::InvalidSignatureSize,
                    end_of_validity: None,
                });
            }
        }
    }

    Ok(result)
}

pub fn verify(data_files: &CardFilesMap, erca_pk: &[u8; 144]) -> Result<VerifyResult> {
    let ca_cert_file =
        data_files.get(&CardFileID::CACertificate).ok_or(Error::VerifyError("Missing CA Certificate.".to_string()))?;
    let card_cert_file =
        data_files.get(&CardFileID::CardCertificate).ok_or(Error::VerifyError("Missing Card Certificate.".to_string()))?;

    let ec_pk_certificate = ECPKCertificate::new(erca_pk)?;
    let ca_certificate = create_certificate_from(ca_cert_file)?;
    let card_certificate = create_certificate_from(card_cert_file)?;

    let ca_decrypted = decrypt_ca_certificate(&ca_certificate, &ec_pk_certificate)?;
    debug!("CA Decrypted: {:?}", ca_decrypted);
    let card_decrypted = decrypt_card_certificate(&card_certificate, &ca_decrypted)?;
    debug!("Card Decrypted: {:?}", card_decrypted);

    let mut result = vec![
        VerifyItem {
            status: VerifyStatus::Valid,
            card_file_id: CardFileID::CACertificate,
            end_of_validity: Some(ca_decrypted.end_of_validity),
        },
        VerifyItem {
            status: VerifyStatus::Valid,
            card_file_id: CardFileID::CardCertificate,
            end_of_validity: Some(card_decrypted.end_of_validity.clone()),
        },
    ];
    let verifed_data = verify_data(data_files, &card_decrypted)?;
    result.extend(verifed_data);

    Ok(VerifyResult { status: VerifyResultStatus::Valid, result })
}
