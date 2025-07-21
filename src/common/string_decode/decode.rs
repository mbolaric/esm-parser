use core::fmt;

use log::trace;

use crate::common::string_decode::{
    code_page::CodePage, iso_8859_1::decode_iso_8859_1, iso_8859_2::decode_iso_8859_2, iso_8859_3::decode_iso_8859_3,
    iso_8859_5::decode_iso_8859_5, iso_8859_7::decode_iso_8859_7, iso_8859_9::decode_iso_8859_9, iso_8859_13::decode_iso_8859_13,
    iso_8859_15::decode_iso_8859_15, iso_8859_16::decode_iso_8859_16, koi8_r::decode_koi8_r, koi8_u::decode_koi8_u,
};

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
        CodePage::Invalid => '\u{FFFD}',
    }
}

pub fn bytes_to_string(bytes: &[u8], enc: &CodePage) -> String {
    let dec_str: String = bytes.iter().map(|&b| decode_byte(b, enc)).collect();
    let ret_str = dec_str.trim_end_matches('\0').trim().to_owned();
    trace!("Bytes: {:?}, Decoded: {:?}, Final: {:?}", bytes, dec_str, ret_str);
    ret_str
}

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

#[derive(Debug)]
pub enum Error {
    InvalidIA5CharacterNotASCII,
    InputTooShortForIA5String,
}

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
