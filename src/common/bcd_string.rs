use crate::{Error, Result};

/// A utility struct for handling BCD (Binary-Coded Decimal) string conversions.
#[derive(Debug)]
pub struct BCDString {}

impl BCDString {
    /// Decodes a byte slice in BCD format into a `String`.
    ///
    /// Each byte in the input slice is treated as two 4-bit nibbles, with each nibble
    /// representing a decimal digit. This function is flexible and handles byte slices of any length.
    ///
    /// # Arguments
    ///
    /// * `bcd` - A byte slice representing the BCD data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded `String` or an `Error` if the input contains invalid BCD data.
    ///
    /// # Examples
    ///
    /// ```
    /// use esm_parser::BCDString;
    /// use esm_parser::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let bcd_data: &[u8] = &[0x12, 0x34, 0x56];
    ///     let string_data = "123456";
    ///
    ///     // Decoding BCD into a string
    ///     let decoded_string = BCDString::decode(bcd_data)?;
    ///     assert_eq!(decoded_string, string_data);
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Decodes a BCD byte slice of a known length into a `String`.
    ///
    /// This strict version ensures that the output string will have a specific length, which is determined
    /// at compile time by the `OUT_LEN` const generic parameter. It returns an error if the input slice
    /// does not correspond to the expected output length.
    ///
    /// # Arguments
    ///
    /// * `bcd` - A byte slice representing the BCD data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded `String` or an `Error` if the input length is incorrect or the data is invalid.
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

    /// Encodes a string of ASCII digits into a BCD-formatted `Vec<u8>`.
    ///
    /// If the input string has an odd number of digits, it is padded with a leading '0'
    /// to ensure that the resulting byte vector is complete.
    ///
    /// # Arguments
    ///
    /// * `val` - A string slice containing only ASCII digits.
    ///
    /// # Returns
    ///
    /// A `Result` containing the BCD-encoded `Vec<u8>` or an `Error` if the input string contains non-digit characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use esm_parser::BCDString;
    /// use esm_parser::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let bcd_data: &[u8] = &[0x12, 0x34, 0x56];
    ///     let string_data = "123456";
    ///
    ///     // Encoding a string into BCD
    ///     let encoded_bcd = BCDString::encode(string_data)?;
    ///     assert_eq!(encoded_bcd, bcd_data);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn encode(val: &str) -> Result<Vec<u8>> {
        if !val.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::InvalidDataParse("Non-digit in BCD string.".to_owned()));
        }

        let mut bytes = Vec::with_capacity(val.len().div_ceil(2));
        let mut chars = val.chars();

        // Handle odd length by processing a prepended '0' first.
        if !val.len().is_multiple_of(2) {
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

    /// Encodes a string of ASCII digits into a BCD-formatted array of a fixed size.
    ///
    /// This strict version requires that the input string's length is exactly twice the size of the output array.
    /// It returns an error if the input string length does not match or if it contains non-digit characters.
    ///
    /// # Arguments
    ///
    /// * `val` - A string slice containing a specific number of ASCII digits.
    ///
    /// # Returns
    ///
    /// A `Result` containing the BCD-encoded array or an `Error` if the input is invalid.
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
