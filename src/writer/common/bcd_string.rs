use crate::{BCDString, Error, Result};

impl BCDString {
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
    /// use esm_parser::{BCDString, Result};
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
}
