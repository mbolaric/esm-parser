use crate::CodePage;
use crate::common::Error;
use crate::string_encoding::iso_8859_1::encode_iso_8859_1;
use crate::string_encoding::iso_8859_2::encode_iso_8859_2;
use crate::string_encoding::iso_8859_3::encode_iso_8859_3;
use crate::string_encoding::iso_8859_5::encode_iso_8859_5;
use crate::string_encoding::iso_8859_7::encode_iso_8859_7;
use crate::string_encoding::iso_8859_9::encode_iso_8859_9;
use crate::string_encoding::iso_8859_13::encode_iso_8859_13;
use crate::string_encoding::iso_8859_15::encode_iso_8859_15;
use crate::string_encoding::iso_8859_16::encode_iso_8859_16;
use crate::string_encoding::koi8_r::encode_koi8_r;
use crate::string_encoding::koi8_u::encode_koi8_u;

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

/// Attempts to encode a single character into a byte using the specified code page.
///
/// Returns an error if the character cannot be losslessly represented in the given
/// code page, or if the code page is invalid.
pub fn try_encode_byte(c: char, enc: &CodePage) -> crate::Result<u8> {
    if *enc == CodePage::Invalid {
        return Err(crate::Error::ValidationError("Cannot encode with invalid code page".to_string()));
    }

    let b = encode_byte(c, enc);
    if b == b'?' && c != '?' {
        return Err(crate::Error::ValidationError(format!(
            "Character '{c}' (U+{:04X}) cannot be represented in code page {enc:?}",
            c as u32
        )));
    }
    Ok(b)
}

/// Converts a `&str` to a `Vec<u8>` using the specified code page, returning an error if any
/// character cannot be losslessly represented in that code page, or if the code page is invalid.
pub fn try_string_to_bytes(s: &str, enc: &CodePage) -> crate::Result<Vec<u8>> {
    if *enc == CodePage::Invalid {
        return Err(crate::Error::ValidationError("Cannot encode with invalid code page".to_string()));
    }
    let mut bytes = Vec::with_capacity(s.len());
    for c in s.chars() {
        bytes.push(try_encode_byte(c, enc)?);
    }
    Ok(bytes)
}

/// Validates and encodes a `&str` into a fixed-size space-padded `[u8; N]` buffer using `enc`.
///
/// Operates entirely on the stack without heap allocations. Returns `Error::ValidationError`
/// if `enc` is `CodePage::Invalid`, any character cannot be losslessly encoded, or the
/// encoded length exceeds `N` bytes.
pub fn encode_code_page_padded<const N: usize>(field: &str, s: &str, enc: &CodePage) -> crate::Result<[u8; N]> {
    if *enc == CodePage::Invalid {
        return Err(crate::Error::ValidationError(format!("{field} has invalid code page")));
    }
    let mut buf = [b' '; N];
    for (count, c) in s.chars().enumerate() {
        if count >= N {
            let total = s.chars().count();
            return Err(crate::Error::ValidationError(format!(
                "{field} encoded length ({total} bytes) exceeds maximum of {N} bytes"
            )));
        }
        buf[count] = try_encode_byte(c, enc)?;
    }
    Ok(buf)
}

/// Validates and encodes an IA5 (ASCII) `&str` into a fixed-size space-padded `[u8; N]` buffer.
///
/// Operates entirely on the stack without heap allocations. Returns `Error::ValidationError`
/// if `s` contains non-ASCII characters or exceeds `N` bytes.
pub fn encode_ia5_padded<const N: usize>(field: &str, s: &str) -> crate::Result<[u8; N]> {
    if !s.is_ascii() {
        return Err(crate::Error::ValidationError(format!("{field} contains non-ASCII (invalid IA5) characters")));
    }
    if s.len() > N {
        return Err(crate::Error::ValidationError(format!("{field} length ({} bytes) exceeds maximum of {N} bytes", s.len())));
    }
    let mut buf = [b' '; N];
    buf[..s.len()].copy_from_slice(s.as_bytes());
    Ok(buf)
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

    #[test]
    fn test_try_string_to_bytes_success() {
        // ISO-8859-1 with valid Latin-1 chars
        let s = "Müller Straße 123";
        let bytes = try_string_to_bytes(s, &CodePage::IsoIec8859_1).unwrap();
        assert_eq!(bytes_to_string(&bytes, &CodePage::IsoIec8859_1), s);

        // ISO-8859-2 with Czech/Slovak chars
        let s2 = "Ahoj Světe! Cože?";
        let bytes2 = try_string_to_bytes(s2, &CodePage::IsoIec8859_2).unwrap();
        assert_eq!(bytes_to_string(&bytes2, &CodePage::IsoIec8859_2), s2);

        // ISO-8859-15 with Euro sign
        let s3 = "Preis: 100 €";
        let bytes3 = try_string_to_bytes(s3, &CodePage::IsoIec8859_15).unwrap();
        assert_eq!(bytes_to_string(&bytes3, &CodePage::IsoIec8859_15), s3);
    }

    #[test]
    fn test_try_string_to_bytes_failures() {
        // Euro sign not in ISO-8859-1
        let err1 = try_string_to_bytes("100 €", &CodePage::IsoIec8859_1);
        assert!(matches!(err1, Err(crate::Error::ValidationError(_))));

        // Euro sign not in ISO-8859-2
        let err2 = try_string_to_bytes("100 €", &CodePage::IsoIec8859_2);
        assert!(matches!(err2, Err(crate::Error::ValidationError(_))));

        // Chinese character not in ISO-8859-1
        let err3 = try_string_to_bytes("你好", &CodePage::IsoIec8859_1);
        assert!(matches!(err3, Err(crate::Error::ValidationError(_))));

        // Invalid code page
        let err4 = try_string_to_bytes("Hello", &CodePage::Invalid);
        assert!(matches!(err4, Err(crate::Error::ValidationError(_))));
    }
}
