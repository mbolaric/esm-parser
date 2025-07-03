mod helpers;

use esm_parser::{HexDisplay, tacho::NationNumericCode};

use crate::helpers::init_logging;

const CERT_LEN: usize = 194;

#[derive(Debug)]
pub struct TachoCertRaw<'a> {
    pub signature: [u8; 128],
    pub public_key_remainder: &'a [u8; 58],
    pub ca_reference: [u8; 8],
}

#[derive(Debug)]
pub struct CAR {
    pub nation_code: NationNumericCode,
    pub key_serial: u8,
    pub additional_info: [u8; 2],
    pub identifier: u8, // constant 0x01
}

pub fn parse_tacho_cert(blob: &[u8]) -> Result<(TachoCertRaw, CAR), &'static str> {
    if blob.len() != CERT_LEN {
        return Err("Invalid certificate length");
    }
    let signature = blob[0..128].try_into().unwrap();
    let public_key_remainder = (&blob[128..186]).try_into().unwrap();
    let ca_reference: [u8; 8] = blob[186..194].try_into().unwrap();

    // Decode CA Reference per KeyIdentifier definition
    // Format: [nation(1), key_serial(1), additional(2), identifier(1), 3 bytes unused?]
    let car = CAR {
        nation_code: ca_reference[0].into(),
        key_serial: ca_reference[1],
        additional_info: [ca_reference[2], ca_reference[3]],
        identifier: ca_reference[4],
    };

    Ok((TachoCertRaw { signature, public_key_remainder, ca_reference }, car))
}

fn decode_signature(sig: &[u8; 128]) -> ([u8; 64], [u8; 64]) {
    let r = sig[0..64].try_into().unwrap();
    let s = sig[64..128].try_into().unwrap();
    (r, s)
}

fn main() {
    init_logging();

    let cert = vec![
        59, 69, 48, 168, 104, 233, 255, 125, 56, 43, 98, 249, 186, 128, 231, 228, 154, 79, 20, 65, 35, 30, 136, 139, 211, 61, 42,
        104, 99, 1, 26, 169, 14, 182, 15, 207, 129, 244, 190, 177, 116, 36, 132, 253, 162, 226, 97, 158, 103, 231, 60, 62, 26,
        239, 82, 22, 173, 190, 129, 55, 137, 26, 34, 228, 146, 228, 110, 128, 241, 189, 38, 15, 31, 109, 169, 202, 35, 220, 243,
        119, 36, 236, 110, 166, 25, 222, 201, 251, 69, 98, 76, 92, 100, 226, 63, 245, 4, 120, 195, 166, 63, 173, 98, 60, 98, 27,
        63, 243, 235, 251, 63, 139, 31, 254, 29, 13, 215, 25, 187, 216, 102, 3, 10, 171, 251, 174, 187, 252, 134, 65, 91, 245,
        103, 123, 118, 58, 73, 177, 65, 242, 166, 182, 179, 187, 74, 169, 219, 18, 151, 165, 159, 198, 193, 99, 94, 148, 193,
        192, 131, 8, 41, 141, 170, 22, 47, 209, 230, 222, 57, 176, 56, 153, 67, 122, 6, 143, 65, 243, 0, 0, 0, 0, 0, 1, 0, 1,
        253, 69, 67, 32, 0, 84, 75, 1,
    ];

    let ret = parse_tacho_cert(&cert.as_slice());
    println!("{:?}", ret);

    let signature = cert[0..128].try_into().unwrap();
    let (r_bytes, s_bytes) = decode_signature(&signature);
    println!("r = {}", r_bytes.to_hex_string());
    println!("s = {}", s_bytes.to_hex_string());
}
