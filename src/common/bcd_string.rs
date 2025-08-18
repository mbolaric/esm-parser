use crate::{Error, Result};

#[derive(Debug)]
pub struct BCDString {}

impl BCDString {
    pub fn decode(bcd: &[u8]) -> Result<String> {
        let mut result = String::new();
        for byte in bcd {
            if (byte >> 4) & 0x0F > 9 || (byte & 0x0F) > 9 {
                return Err(Error::InvalidDataParse("Invalid BCDString input data.".to_owned()));
            }
            result.push(char::from(b'0' + ((byte >> 4) & 0x0F)));
            result.push(char::from(b'0' + (byte & 0x0F)));
        }
        Ok(result)
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
    }

    #[test]
    fn test_multiple_bytes_valid() {
        // 0x12 0x34 0x56 → "123456"
        let input = [0x12, 0x34, 0x56];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "123456");
    }

    #[test]
    fn test_leading_zero() {
        // 0x01 → "01"
        let input = [0x01];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "01");
    }

    #[test]
    fn test_trailing_zero() {
        // 0x90 → "90"
        let input = [0x90];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "90");
    }

    #[test]
    fn test_empty_input() {
        let input: [u8; 0] = [];
        let decoded = BCDString::decode(&input).unwrap();
        assert_eq!(decoded, "");
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
