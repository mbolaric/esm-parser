use core::fmt;

use log::trace;

use crate::common::string_decode::{
    code_page::CodePage, iso_8859_1::decode_iso_8859_1, iso_8859_2::decode_iso_8859_2, iso_8859_3::decode_iso_8859_3,
    iso_8859_5::decode_iso_8859_5, iso_8859_7::decode_iso_8859_7, iso_8859_9::decode_iso_8859_9, iso_8859_13::decode_iso_8859_13,
    iso_8859_15::decode_iso_8859_15, iso_8859_16::decode_iso_8859_16, koi8_r::decode_koi8_r, koi8_u::decode_koi8_u,
};

/// Decodes a single byte into a character based on the specified code page.
///
/// This function acts as a dispatcher, calling the appropriate decoding function
/// for the given `CodePage`. If the code page is invalid, it returns the Unicode
/// replacement character (`\u{FFFD}`).
///
/// # Arguments
///
/// * `byte` - The byte to decode.
/// * `enc` - The `CodePage` to use for decoding.
///
/// # Returns
///
/// The decoded `char`.
fn decode_byte(byte: u8, enc: &CodePage) -> char {
    match enc {
        CodePage::IsoIec8859_1 => decode_iso_8859_1(byte),
        CodePage::IsoIec8859_2 => decode_iso_8859_2(byte),
        CodePage::IsoIec8859_3 => decode_iso_8859_3(byte),
        CodePage::IsoIec8859_5 => decode_iso_8859_5(byte),
        CodePage::IsoIec8859_7 => decode_iso_8859_7(byte),
        CodePage::IsoIec8859_9 => decode_iso_8859_9(byte),
        CodePage::IsoIec8859_13 => decode_iso_8859_13(byte),
        CodePage::IsoIec8859_15 => decode_iso_8859_15(byte),
        CodePage::IsoIec8859_16 => decode_iso_8859_16(byte),
        CodePage::Koi8R => decode_koi8_r(byte),
        CodePage::Koi8U => decode_koi8_u(byte),
        CodePage::Invalid => '\u{FFFD}', // Unicode replacement character for invalid encodings
    }
}

/// Converts a byte slice to a `String` using the specified code page.
///
/// This function iterates over the byte slice, decodes each byte into a character,
/// and collects them into a `String`. It then trims any trailing null characters
/// and whitespace from the resulting string.
///
/// # Arguments
///
/// * `bytes` - The byte slice to decode.
/// * `enc` - The `CodePage` to use for decoding.
///
/// # Returns
///
/// The decoded and trimmed `String`.
///
/// # Examples
///
/// ```
/// use esm_parser::{bytes_to_string, CodePage};
///
/// // Example of decoding a byte slice using ISO-8859-1
/// let bytes: &[u8] = &[0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
/// let decoded_string = bytes_to_string(bytes, &CodePage::IsoIec8859_1);
/// assert_eq!(decoded_string, "Hello");
///
/// // Example with a non-ASCII character in ISO-8859-15 (Euro sign)
/// let bytes_with_euro: &[u8] = &[0xA4];
/// let decoded_euro = bytes_to_string(bytes_with_euro, &CodePage::IsoIec8859_15);
/// assert_eq!(decoded_euro, "€");
/// ```
pub fn bytes_to_string(bytes: &[u8], enc: &CodePage) -> String {
    let dec_str: String = bytes.iter().map(|&b| decode_byte(b, enc)).collect();
    let ret_str = dec_str.trim_end_matches('\0').trim().to_owned();
    trace!("Bytes: {bytes:?}, Decoded: {dec_str:?}, Final: {ret_str:?}");
    ret_str
}

/// Converts a byte slice to a `String`, assuming it is an IA5 (ASCII) fixed-length string.
///
/// This function validates that all bytes in the input slice are valid ASCII characters.
/// If the validation passes, it converts the byte slice to a `String` and trims trailing
/// null characters and whitespace.
///
/// # Arguments
///
/// * `input` - The byte slice to decode.
///
/// # Returns
///
/// A `Result` containing the decoded `String` or an `Error` if the input is not valid IA5.
pub fn bytes_to_ia5_fix_string(input: &[u8]) -> Result<String, Error> {
    if input.is_empty() {
        return Ok(String::new());
    }

    // Validate it's ASCII (IA5)
    if !input.iter().all(|&b| b.is_ascii()) {
        return Err(Error::InvalidIA5CharacterNotASCII);
    }

    match String::from_utf8(input.to_vec()) {
        Ok(s) => Ok(s.trim_end_matches('\0').trim().to_string()),
        Err(_) => Err(Error::InvalidIA5CharacterNotASCII),
    }
}

/// Defines the possible errors that can occur during string decoding.
#[derive(Debug)]
pub enum Error {
    /// Error indicating that a character is not a valid IA5 (ASCII) character.
    InvalidIA5CharacterNotASCII,
    /// Error indicating that the input is too short for a fixed-length IA5 string.
    InputTooShortForIA5String,
}

/// Implements the `Display` trait for the `Error` enum to provide a user-friendly error message.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_8859_1() {
        for b in 0u8..=255 {
            assert_eq!(decode_iso_8859_1(b), b as char);
        }
    }

    #[test]
    fn test_iso_8859_2_sample() {
        assert_eq!(decode_iso_8859_2(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_2(0xA1), '\u{0104}');
        assert_eq!(decode_iso_8859_2(0xA2), '\u{02D8}');
        assert_eq!(decode_iso_8859_2(0xA3), '\u{0141}');
    }

    #[test]
    fn test_iso_8859_3_sample() {
        assert_eq!(decode_iso_8859_3(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_3(0xA1), '\u{0126}');
        assert_eq!(decode_iso_8859_3(0xA2), '\u{02D8}');
        assert_eq!(decode_iso_8859_3(0xA3), '\u{00A3}');
    }

    #[test]
    fn test_iso_8859_5_sample() {
        assert_eq!(decode_iso_8859_5(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_5(0xA1), '\u{0401}');
        assert_eq!(decode_iso_8859_5(0xA2), '\u{0402}');
        assert_eq!(decode_iso_8859_5(0xA3), '\u{0403}');
    }

    #[test]
    fn test_iso_8859_7_sample() {
        assert_eq!(decode_iso_8859_7(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_7(0xA1), '\u{2018}');
        assert_eq!(decode_iso_8859_7(0xA2), '\u{2019}');
        assert_eq!(decode_iso_8859_7(0xA3), '\u{00A3}');
    }

    #[test]
    fn test_iso_8859_9_sample() {
        assert_eq!(decode_iso_8859_9(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_9(0xA1), '\u{00A1}');
        assert_eq!(decode_iso_8859_9(0xA2), '\u{00A2}');
        assert_eq!(decode_iso_8859_9(0xA3), '\u{00A3}');
    }

    #[test]
    fn test_iso_8859_13_sample() {
        assert_eq!(decode_iso_8859_13(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_13(0xA1), '\u{201D}');
        assert_eq!(decode_iso_8859_13(0xA2), '\u{00A2}');
        assert_eq!(decode_iso_8859_13(0xA3), '\u{00A3}');
    }

    #[test]
    fn test_iso_8859_15_sample() {
        assert_eq!(decode_iso_8859_15(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_15(0xA1), '\u{00A1}');
        assert_eq!(decode_iso_8859_15(0xA2), '\u{00A2}');
        assert_eq!(decode_iso_8859_15(0xA3), '\u{00A3}');
    }

    #[test]
    fn test_iso_8859_16_sample() {
        assert_eq!(decode_iso_8859_16(0xA0), '\u{00A0}');
        assert_eq!(decode_iso_8859_16(0xA1), '\u{0104}');
        assert_eq!(decode_iso_8859_16(0xA2), '\u{0105}');
        assert_eq!(decode_iso_8859_16(0xA3), '\u{0141}');
    }

    #[test]
    fn test_koi8_r_sample() {
        assert_eq!(decode_koi8_r(0xA0), '\u{2550}');
        assert_eq!(decode_koi8_r(0xA1), '\u{2551}');
        assert_eq!(decode_koi8_r(0xA2), '\u{2552}');
        assert_eq!(decode_koi8_r(0xA3), '\u{0451}');
    }

    #[test]
    fn test_koi8_u_sample() {
        assert_eq!(decode_koi8_u(0xA0), '\u{2550}');
        assert_eq!(decode_koi8_u(0xA1), '\u{2551}');
        assert_eq!(decode_koi8_u(0xA2), '\u{2552}');
        assert_eq!(decode_koi8_u(0xA3), '\u{0451}');
    }
}
