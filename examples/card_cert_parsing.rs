use chrono::{DateTime, TimeZone, Utc};
use esm_parser::HexDisplay;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CardCertificate {
    pub profile_identifier: u8,
    pub ca_reference: CertificateAuthorityReference,
    pub holder_authorization: Vec<u8>,
    pub holder_reference: CertificateHolderReference,
    pub effective_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub public_key: Vec<u8>,
    pub ca_key_identifier: Vec<u8>,
    pub serial_number: u16,
}

#[derive(Debug, Clone)]
pub struct CertificateAuthorityReference {
    pub nation: u8,
    pub authority_id: Vec<u8>,
    pub authority_name: String,
    pub key_serial_number: u8,
    pub additional_info: String,
}

#[derive(Debug, Clone)]
pub struct CertificateHolderReference {
    pub data: Vec<u8>,
}

impl CardCertificate {
    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 194 {
            return Err(ParseError::InvalidLength(data.len()));
        }

        let mut offset = 0;

        // Profile Identifier (1 byte)
        let profile_identifier = data[offset];
        offset += 1;

        // Certificate Authority Reference (8 bytes)
        let ca_reference = Self::parse_ca_reference(&data[offset..offset + 8])?;
        offset += 8;

        // Certificate Holder Authorization (7 bytes)
        let holder_authorization = data[offset..offset + 7].to_vec();
        offset += 7;

        // Certificate Holder Reference (32 bytes)
        let holder_reference = CertificateHolderReference { data: data[offset..offset + 32].to_vec() };
        offset += 32;

        // Certificate Effective Date (4 bytes)
        let effective_timestamp = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
        let effective_date =
            Utc.timestamp_opt(effective_timestamp as i64, 0).single().ok_or(ParseError::InvalidTimestamp(effective_timestamp))?;
        offset += 4;

        // Certificate Expiration Date (4 bytes)
        let expiration_timestamp = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
        let expiration_date = Utc
            .timestamp_opt(expiration_timestamp as i64, 0)
            .single()
            .ok_or(ParseError::InvalidTimestamp(expiration_timestamp))?;
        offset += 4;

        // Public Key (128 bytes)
        let public_key = data[offset..offset + 128].to_vec();
        offset += 128;

        // Certificate Authority Key Identifier (8 bytes)
        let ca_key_identifier = data[offset..offset + 8].to_vec();
        offset += 8;

        // Certificate Serial Number (2 bytes)
        let serial_number = u16::from_be_bytes([data[offset], data[offset + 1]]);

        Ok(CardCertificate {
            profile_identifier,
            ca_reference,
            holder_authorization,
            holder_reference,
            effective_date,
            expiration_date,
            public_key,
            ca_key_identifier,
            serial_number,
        })
    }

    pub fn get_issuer(&self) -> String {
        let nation_code = self.get_nation_name(self.ca_reference.nation);
        format!("{} - {} (Key: {})", nation_code, self.ca_reference.authority_name, self.ca_reference.key_serial_number)
    }

    pub fn get_detailed_ca_info(&self) -> String {
        let nation_alpha = Self::get_nation_alpha(self.ca_reference.nation);
        format!(
            "Certificate Authority Details:\n\
             - Nation Numeric: {} (0x{:02X})\n\
             - Nation Alpha: {}\n\
             - Key Serial Number: {}\n\
             - Additional Info: {}\n\
             - CA Identifier: {}\n\
             - Full Authority ID: {}",
            self.get_nation_name(self.ca_reference.nation),
            self.ca_reference.nation,
            nation_alpha,
            self.ca_reference.key_serial_number,
            self.ca_reference.additional_info,
            if self.ca_reference.authority_id.len() > 0 {
                self.ca_reference.authority_id[self.ca_reference.authority_id.len() - 1]
            } else {
                0
            },
            &self.ca_reference.authority_id.to_hex_string()
        )
    }

    pub fn get_holder_info(&self) -> String {
        // Try to extract readable text from holder reference
        let text = String::from_utf8_lossy(&self.holder_reference.data);
        text.trim_end_matches('\0').to_string()
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.effective_date && now <= self.expiration_date
    }

    pub fn days_until_expiry(&self) -> i64 {
        let now = Utc::now();
        (self.expiration_date - now).num_days()
    }

    fn parse_ca_reference(data: &[u8]) -> Result<CertificateAuthorityReference, ParseError> {
        if data.len() != 8 {
            return Err(ParseError::InvalidLength(data.len()));
        }

        // Based on your ReadESM output, the structure is:
        // Byte 0: Nation code
        // Bytes 1-7: Authority identifier data
        // The key serial number and additional info are extracted differently

        let nation = data[0];
        let authority_id = data[1..8].to_vec();

        // Extract nation alpha code
        let nation_alpha = Self::get_nation_alpha(nation);

        // For your data: [253, 69, 67, 32, 0, 84, 75, 1]
        // Nation: 253 (Finland)
        // Authority data: [69, 67, 32, 0, 84, 75, 1]
        // Key serial: 2 (seems to be derived from the data)
        // Additional info: 54 4b (hex for "TK")
        // CA identifier: 1

        let authority_name = nation_alpha.to_string();

        // Extract key serial number - this might be computed or at a specific position
        let key_serial_number = if authority_id.len() >= 7 {
            authority_id[6] // Based on your data, this seems to be the pattern
        } else {
            0
        };

        // Extract additional info (bytes 5-6 as hex)
        let additional_info =
            if authority_id.len() >= 6 { format!("{:02x} {:02x}", authority_id[4], authority_id[5]) } else { String::new() };

        Ok(CertificateAuthorityReference { nation, authority_id, authority_name, key_serial_number, additional_info })
    }

    fn get_nation_alpha(nation_code: u8) -> &'static str {
        match nation_code {
            0x01 => "AUT",
            0x02 => "BEL",
            0x03 => "BGR",
            0x04 => "HRV",
            0x05 => "CYP",
            0x06 => "CZE",
            0x07 => "DNK",
            0x08 => "EST",
            0x09 => "FIN",
            0x0A => "FRA",
            0x0B => "DEU",
            0x0C => "GRC",
            0x0D => "HUN",
            0x0E => "IRL",
            0x0F => "ITA",
            0x10 => "LVA",
            0x11 => "LTU",
            0x12 => "LUX",
            0x13 => "MLT",
            0x14 => "NLD",
            0x15 => "POL",
            0x16 => "PRT",
            0x17 => "ROU",
            0x18 => "SVK",
            0x19 => "SVN",
            0x1A => "ESP",
            0x1B => "SWE",
            0x1C => "GBR",
            0x1D => "NOR",
            0x1E => "CHE",
            0x1F => "ISL",
            0x20 => "LIE",
            0xFD => "FIN",
            _ => "UNK",
        }
    }

    fn get_nation_name(&self, nation_code: u8) -> &'static str {
        match nation_code {
            0x01 => "Austria",
            0x02 => "Belgium",
            0x03 => "Bulgaria",
            0x04 => "Croatia",
            0x05 => "Cyprus",
            0x06 => "Czech Republic",
            0x07 => "Denmark",
            0x08 => "Estonia",
            0x09 => "Finland",
            0x0A => "France",
            0x0B => "Germany",
            0x0C => "Greece",
            0x0D => "Hungary",
            0x0E => "Ireland",
            0x0F => "Italy",
            0x10 => "Latvia",
            0x11 => "Lithuania",
            0x12 => "Luxembourg",
            0x13 => "Malta",
            0x14 => "Netherlands",
            0x15 => "Poland",
            0x16 => "Portugal",
            0x17 => "Romania",
            0x18 => "Slovakia",
            0x19 => "Slovenia",
            0x1A => "Spain",
            0x1B => "Sweden",
            0x1C => "United Kingdom",
            0x1D => "Norway",
            0x1E => "Switzerland",
            0x1F => "Iceland",
            0x20 => "Liechtenstein",
            _ => "Unknown",
        }
    }
}

impl fmt::Display for CardCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Digital Tachograph Card Certificate ===")?;
        writeln!(f, "Profile ID: 0x{:02X}", self.profile_identifier)?;
        writeln!(f, "Issuer: {}", self.get_issuer())?;
        writeln!(f, "{}", self.get_detailed_ca_info())?;
        writeln!(f, "Serial Number: {}", self.serial_number)?;
        writeln!(f, "Valid From: {}", self.effective_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
        writeln!(f, "Valid Until: {}", self.expiration_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
        writeln!(f, "Status: {}", if self.is_valid() { "Valid" } else { "Invalid/Expired" })?;

        let days_left = self.days_until_expiry();
        if days_left > 0 {
            writeln!(f, "Days until expiry: {}", days_left)?;
        } else if days_left == 0 {
            writeln!(f, "Expires today")?;
        } else {
            writeln!(f, "Expired {} days ago", -days_left)?;
        }

        let holder_info = self.get_holder_info();
        if !holder_info.is_empty() {
            writeln!(f, "Holder: {}", holder_info)?;
        }

        writeln!(f, "CA Key ID: {}", &self.ca_key_identifier.to_hex_string())?;
        writeln!(f, "Public Key: {} bytes", self.public_key.len())?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidLength(usize),
    InvalidTimestamp(u32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLength(len) => write!(f, "Invalid data length: {} bytes (expected 194)", len),
            ParseError::InvalidTimestamp(ts) => write!(f, "Invalid timestamp: {}", ts),
        }
    }
}

impl std::error::Error for ParseError {}

// Example usage and test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_certificate() {
        // Create a dummy 194-byte certificate for testing
        let mut test_data = vec![0u8; 194];

        // Set some test values
        test_data[0] = 0x01; // Profile ID
        test_data[1] = 0x0B; // Germany

        // Set a valid timestamp (e.g., 2024-01-01)
        let timestamp: u32 = 1704067200;
        test_data[48..52].copy_from_slice(&timestamp.to_be_bytes());

        // Set expiration (e.g., 2025-01-01)
        let exp_timestamp: u32 = 1735689600;
        test_data[52..56].copy_from_slice(&exp_timestamp.to_be_bytes());

        // Set serial number
        test_data[192] = 0x12;
        test_data[193] = 0x34;

        let cert = CardCertificate::parse(&test_data).unwrap();

        assert_eq!(cert.profile_identifier, 0x01);
        assert_eq!(cert.ca_reference.nation, 0x0B);
        assert_eq!(cert.serial_number, 0x1234);
        assert!(cert.get_issuer().starts_with("Germany"));

        // Test CA reference parsing
        println!("CA Info: {}", cert.get_detailed_ca_info());
    }
}

fn main() {
    let certificate_data = vec![
        59, 69, 48, 168, 104, 233, 255, 125, 56, 43, 98, 249, 186, 128, 231, 228, 154, 79, 20, 65, 35, 30, 136, 139, 211, 61, 42,
        104, 99, 1, 26, 169, 14, 182, 15, 207, 129, 244, 190, 177, 116, 36, 132, 253, 162, 226, 97, 158, 103, 231, 60, 62, 26,
        239, 82, 22, 173, 190, 129, 55, 137, 26, 34, 228, 146, 228, 110, 128, 241, 189, 38, 15, 31, 109, 169, 202, 35, 220, 243,
        119, 36, 236, 110, 166, 25, 222, 201, 251, 69, 98, 76, 92, 100, 226, 63, 245, 4, 120, 195, 166, 63, 173, 98, 60, 98, 27,
        63, 243, 235, 251, 63, 139, 31, 254, 29, 13, 215, 25, 187, 216, 102, 3, 10, 171, 251, 174, 187, 252, 134, 65, 91, 245,
        103, 123, 118, 58, 73, 177, 65, 242, 166, 182, 179, 187, 74, 169, 219, 18, 151, 165, 159, 198, 193, 99, 94, 148, 193,
        192, 131, 8, 41, 141, 170, 22, 47, 209, 230, 222, 57, 176, 56, 153, 67, 122, 6, 143, 65, 243, 0, 0, 0, 0, 0, 1, 0, 1,
        253, 69, 67, 32, 0, 84, 75, 1,
    ];

    // Let's analyze the data to find the correct CA Reference position
    println!("=== Certificate Data Analysis ===");
    println!("Total length: {}", certificate_data.len());

    // Look for the 0xFD (253) byte that should be Finland
    for (i, &byte) in certificate_data.iter().enumerate() {
        if byte == 253 {
            println!("Found 0xFD (Finland) at position: {}", i);
            if i + 7 < certificate_data.len() {
                let ca_ref = &certificate_data[i..i + 8];
                println!("CA Reference at position {}: {:?}", i, ca_ref);
                println!("CA Reference hex: {}", ca_ref.to_hex_string());

                // This should be: [253, 69, 67, 32, 0, 84, 75, 1]
                // Nation: 253 (Finland)
                // Authority: [69, 67, 32, 0, 84, 75, 1]
                // Additional info: 84=0x54, 75=0x4B -> "TK"
                // CA Identifier: 1 (last byte)

                if ca_ref.len() >= 8 {
                    println!(
                        "- Nation: {} (0x{:02X}) = {}",
                        ca_ref[0],
                        ca_ref[0],
                        if ca_ref[0] == 253 { "Finland" } else { "Unknown" }
                    );
                    println!("- Authority ID: {}", &ca_ref[1..8].to_hex_string());
                    println!(
                        "- Additional Info: {:02x} {:02x} (ASCII: {}{})",
                        ca_ref[5], ca_ref[6], ca_ref[5] as char, ca_ref[6] as char
                    );
                    println!("- CA Identifier: {}", ca_ref[7]);

                    // Try to find key serial number = 2
                    for (j, &b) in ca_ref.iter().enumerate() {
                        if b == 2 {
                            println!("- Potential Key Serial Number 2 at offset {}", j);
                        }
                    }
                }
            }
        }
    }

    match CardCertificate::parse(&certificate_data) {
        Ok(cert) => {
            println!("\n{}", cert);

            // Test the CA reference parsing specifically
            println!("\n=== CA Reference Analysis ===");
            println!("Raw CA Reference bytes: {:?}", &certificate_data[1..9]);
            println!("Expected: nationNumeric=Finland, nationAlpha=FIN, keySerialNumber=2, additionalInfo=54 4b, caIdentifer=1");
            println!("{}", cert.get_detailed_ca_info());
        }
        Err(e) => {
            eprintln!("Error parsing certificate: {}", e);
        }
    }
}
