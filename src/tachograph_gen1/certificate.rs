use serde::Serialize;

use crate::{CodePage, HexDisplay, Readable, bytes_to_string, tacho::NationNumeric};

#[derive(Debug, Serialize)]
pub struct ParsedCertificationAuthorityReference {
    pub nation_numeric_code: u8,
    pub nation_numeric: NationNumeric,
    pub nation_alpha: String,
    pub key_serial_number: u8,
    pub additional_info: String,
    pub ca_identifier: u8,
}

// println!("nationNumeric: {}", nation_numeric);
// println!("nationAlpha: {}", nation_alpha);
// println!("keySerialNumber: {}", key_serial);
// println!("additionalInfo: {}", encode_upper(additional_info));
// println!("caIdentifier: {}", ca_id);

#[derive(Debug, Serialize)]
pub struct Certificate {
    pub signature: Vec<u8>,
    #[serde(rename = "publicKeyRemainder")]
    pub public_key_remainder: Vec<u8>,
    #[serde(rename = "certificationAuthorityReference")]
    pub certification_authority_reference: Vec<u8>,
    #[serde(rename = "parsedCAReference")]
    pub parsed_ca_reference: ParsedCertificationAuthorityReference,
}

impl Certificate {
    fn parse_certification_authority_reference(ca_reference: &[u8]) -> ParsedCertificationAuthorityReference {
        let nation_numeric_code = ca_reference[0];
        let nation_numeric: NationNumeric = nation_numeric_code.into();
        let nation_alpha = bytes_to_string(&ca_reference[1..3], &CodePage::IsoIec8859_1).trim().to_owned();
        let key_serial_number = ca_reference[4];
        let additional_info = ca_reference[5..7].to_hex_string_with_sep(" ");
        let ca_identifier = ca_reference[7];

        ParsedCertificationAuthorityReference {
            nation_numeric_code,
            nation_numeric,
            nation_alpha,
            key_serial_number,
            additional_info,
            ca_identifier,
        }
    }
}

impl Readable<Certificate> for Certificate {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<Certificate> {
        // Size = 194
        let signature = reader.read_into_vec(128)?;
        let public_key_remainder = reader.read_into_vec(58)?;
        // https://dtc.jrc.ec.europa.eu/dtc_public_key_certificates_dt.php.html
        let certification_authority_reference = reader.read_into_vec(8)?;
        let parsed_ca_reference = Certificate::parse_certification_authority_reference(&certification_authority_reference);

        Ok(Self { signature, public_key_remainder, certification_authority_reference, parsed_ca_reference })
    }
}
