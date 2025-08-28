use crate::{Error, Result};

#[derive(Debug)]
pub struct BCDString {}

impl BCDString {
    /// Decodes a BCD byte slice into a String.
    /// This version is flexible and handles any input length.
    pub fn decode(bcd: &[u8]) -> Result<String> {
        // Optimization: Pre-allocate the string to the final size.
        let mut result = String::with_capacity(bcd.len() * 2);

        for &byte in bcd {
            let high = (byte >> 4) & 0x0F;
            let low = byte & 0x0F;

            if high > 9 || low > 9 {
                return Err(Error::InvalidDataParse("Invalid BCDString input data.".to_owned()));
            }

            // Optimization: Pushing bytes is faster than `char::from`.
            result.push((b'0' + high) as char);
            result.push((b'0' + low) as char);
        }
        Ok(result)
    }

    /// Decodes a BCD byte slice of a known length into a String.
    /// This strict version ensures the output string will have exactly `OUT_LEN` characters.
    pub fn decode_strict<const OUT_LEN: usize>(bcd: &[u8]) -> Result<String> {
        if bcd.len() * 2 != OUT_LEN {
            return Err(Error::InvalidDataParse(format!(
                "Invalid BCDString input length: expected {} bytes for {} digits, got {}",
                OUT_LEN / 2,
                OUT_LEN,
                bcd.len()
            )));
        }
        Self::decode(bcd)
    }

    /// Encodes a string of ASCII digits into a BCD byte vector.
    /// Pads with a leading '0' if the input string has an odd number of digits.
    pub fn encode(val: &str) -> Result<Vec<u8>> {
        if !val.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::InvalidDataParse("Non-digit in BCD string.".to_owned()));
        }

        let mut bytes = Vec::with_capacity(val.len().div_ceil(2));
        let mut chars = val.chars();

        // Handle odd length by processing a prepended '0' first.
        if val.len() % 2 != 0 {
            let low = (chars.next().unwrap() as u8) - b'0';
            bytes.push(low);
        }

        while let Some(high_char) = chars.next() {
            let low_char = chars.next().unwrap(); // Safe to unwrap due to length handling
            let high = (high_char as u8) - b'0';
            let low = (low_char as u8) - b'0';
            bytes.push((high << 4) | low);
        }

        Ok(bytes)
    }

    /// Encodes a string of ASCII digits into a BCD array of a fixed size.
    /// The input string must contain exactly `OUT_LEN * 2` digits.
    pub fn encode_strict<const OUT_LEN: usize>(val: &str) -> Result<[u8; OUT_LEN]> {
        if val.len() != OUT_LEN * 2 {
            return Err(Error::InvalidDataParse(format!(
                "Invalid BCDString input length: expected {} digits, got {}",
                OUT_LEN * 2,
                val.len()
            )));
        }

        if !val.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::InvalidDataParse("Non-digit in BCD string.".to_owned()));
        }

        let mut bytes = [0u8; OUT_LEN];
        let mut chars = val.chars();

        for byte in bytes.iter_mut() {
            let high_char = chars.next().unwrap();
            let low_char = chars.next().unwrap();
            let high = (high_char as u8) - b'0';
            let low = (low_char as u8) - b'0';
            *byte = (high << 4) | low;
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_byte_valid() {
        // 0x12 → "12"
        let input = [0x12];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "12");

        let decoded = BCDString::decode_strict::<2>(&input).unwrap();
        assert_eq!(decoded, "12");

        let encoded = BCDString::encode(&decoded).unwrap();
        assert_eq!(encoded, input);

        let encoded = BCDString::encode_strict::<1>(&decoded).unwrap();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_multiple_bytes_valid() {
        // 0x12 0x34 0x56 → "123456"
        let input = [0x12, 0x34, 0x56];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "123456");

        let decoded = BCDString::decode_strict::<6>(&input).unwrap();
        assert_eq!(decoded, "123456");

        let encoded = BCDString::encode(&decoded).unwrap();
        assert_eq!(encoded, input);

        let encoded = BCDString::encode_strict::<3>(&decoded).unwrap();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_leading_zero() {
        // 0x01 → "01"
        let input = [0x01];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "01");

        let decoded = BCDString::decode_strict::<2>(&input).unwrap();
        assert_eq!(decoded, "01");

        let encoded = BCDString::encode(&decoded).unwrap();
        assert_eq!(encoded, input);

        let encoded = BCDString::encode_strict::<1>(&decoded).unwrap();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_trailing_zero() {
        // 0x90 → "90"
        let input = [0x90];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "90");

        let decoded = BCDString::decode_strict::<2>(&input).unwrap();
        assert_eq!(decoded, "90");

        let encoded = BCDString::encode(&decoded).unwrap();
        assert_eq!(encoded, input);

        let encoded = BCDString::encode_strict::<1>(&decoded).unwrap();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_empty_input() {
        let input: [u8; 0] = [];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "");

        let encoded = BCDString::encode(&decoded).unwrap();
        assert_eq!(encoded, input);

        let encoded = BCDString::encode_strict::<0>(&decoded).unwrap();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_invalid_high_nibble() {
        // 0xAB → high nibble A (10) is invalid
        let input = [0xAB];
        let err = BCDString::decode(&input).unwrap_err();
        match err {
            Error::InvalidDataParse(msg) => {
                assert_eq!(msg, "Invalid BCDString input data.");
            }
            _ => panic!("Expected InvalidDataParse error"),
        }
    }

    #[test]
    fn test_invalid_low_nibble() {
        // 0x1F → low nibble F (15) is invalid
        let input = [0x1F];
        let err = BCDString::decode(&input).unwrap_err();
        match err {
            Error::InvalidDataParse(msg) => {
                assert_eq!(msg, "Invalid BCDString input data.");
            }
            _ => panic!("Expected InvalidDataParse error"),
        }
    }
}
