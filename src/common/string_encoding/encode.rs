use crate::{
    CodePage,
    common::Error,
    string_encoding::{
        iso_8859_1::encode_iso_8859_1, iso_8859_2::encode_iso_8859_2, iso_8859_3::encode_iso_8859_3,
        iso_8859_5::encode_iso_8859_5, iso_8859_7::encode_iso_8859_7, iso_8859_9::encode_iso_8859_9,
        iso_8859_13::encode_iso_8859_13, iso_8859_15::encode_iso_8859_15, iso_8859_16::encode_iso_8859_16, koi8_r::encode_koi8_r,
        koi8_u::encode_koi8_u,
    },
};

/// Converts a `&str` to a `Vec<u8>` using the specified code page.
///
/// This function iterates over the characters of the string, encodes each character into a byte,
/// and collects them into a `Vec<u8>`.
///
/// # Arguments
///
/// * `s` - The `&str` to encode.
/// * `enc` - The `CodePage` to use for encoding.
///
/// # Returns
///
/// The encoded `Vec<u8>`.
pub fn string_to_bytes(s: &str, enc: &CodePage) -> Vec<u8> {
    s.chars().map(|c| encode_byte(c, enc)).collect()
}

/// Encodes a single character into a byte based on the specified code page.
///
/// This function acts as a dispatcher, calling the appropriate encoding function
/// for the given `CodePage`. If the code page is invalid or the character is not
/// supported, it returns the ASCII code for `?` (`0x3F`).
///
/// # Arguments
///
/// * `c` - The character to encode.
/// * `enc` - The `CodePage` to use for encoding.
///
/// # Returns
///
/// The encoded `u8`.
fn encode_byte(c: char, enc: &CodePage) -> u8 {
    match enc {
        CodePage::IsoIec8859_1 => encode_iso_8859_1(c),
        CodePage::IsoIec8859_2 => encode_iso_8859_2(c),
        CodePage::IsoIec8859_3 => encode_iso_8859_3(c),
        CodePage::IsoIec8859_5 => encode_iso_8859_5(c),
        CodePage::IsoIec8859_7 => encode_iso_8859_7(c),
        CodePage::IsoIec8859_9 => encode_iso_8859_9(c),
        CodePage::IsoIec8859_13 => encode_iso_8859_13(c),
        CodePage::IsoIec8859_15 => encode_iso_8859_15(c),
        CodePage::IsoIec8859_16 => encode_iso_8859_16(c),
        CodePage::Koi8R => encode_koi8_r(c),
        CodePage::Koi8U => encode_koi8_u(c),
        _ => b'?',
    }
}

/// Converts a `&str` to a `Vec<u8>`, assuming it is an IA5 (ASCII) fixed-length string.
///
/// This function validates that all characters in the input string are valid ASCII characters.
/// If the validation passes, it converts the string to a `Vec<u8>`.
///
/// # Arguments
///
/// * `s` - The `&str` to encode.
///
/// # Returns
///
/// A `Result` containing the encoded `Vec<u8>` or an `Error` if the input is not valid IA5.
pub fn ia5_fix_string_to_bytes(s: &str) -> Result<Vec<u8>, Error> {
    if !s.is_ascii() {
        return Err(Error::InvalidIA5StringNotASCII);
    }
    Ok(s.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes_to_string;

    #[test]
    fn test_string_to_bytes_iso8859_1() {
        let s = "Hello World!";
        let bytes = string_to_bytes(s, &CodePage::IsoIec8859_1);
        let s2 = bytes_to_string(&bytes, &CodePage::IsoIec8859_1);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_string_to_bytes_iso8859_2() {
        let s = "Ahoj Světe!";
        let bytes = string_to_bytes(s, &CodePage::IsoIec8859_2);
        let s2 = bytes_to_string(&bytes, &CodePage::IsoIec8859_2);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_ia5_fix_string_to_bytes() {
        let s = "Hello World!";
        let bytes = ia5_fix_string_to_bytes(s).unwrap();
        assert_eq!(s.as_bytes(), bytes.as_slice());
    }

    #[test]
    fn test_ia5_fix_string_to_bytes_invalid() {
        let s = "Ahoj Světe!";
        let result = ia5_fix_string_to_bytes(s);
        assert!(result.is_err());
    }
}
