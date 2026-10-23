use bp256::BrainpoolP256r1;
use bp256::r1::ecdsa::Signature;
use ecdsa::SigningKey;
use sha2::{Digest, Sha256};
use signature::hazmat::PrehashSigner;

use super::card_verifiable_certificate::{
    CAR_SIZE, CHR_SIZE, CPI_VERSION_1, GEN2_CERTIFICATE_SIZE, SEC1_UNCOMPRESSED_P256_POINT_SIZE,
};

pub(super) fn signing_key(last_byte: u8) -> SigningKey<BrainpoolP256r1> {
    let mut scalar = [0u8; 32];
    scalar[31] = last_byte;
    SigningKey::from_slice(&scalar).unwrap()
}

pub(super) fn sign(signer: &SigningKey<BrainpoolP256r1>, payload: &[u8]) -> Vec<u8> {
    let signature: Signature = signer.sign_prehash(&Sha256::digest(payload)).unwrap();
    signature.to_bytes().as_slice().to_vec()
}

pub(super) fn certificate_for(
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
