use serde::Serialize;

/// Represents a code page for character encoding.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum CodePage {
    /// ISO/IEC 8859-1 (Latin-1).
    IsoIec8859_1 = 1,
    /// ISO/IEC 8859-2 (Latin-2).
    IsoIec8859_2 = 2,
    /// ISO/IEC 8859-3 (Latin-3).
    IsoIec8859_3 = 3,
    /// ISO/IEC 8859-5 (Cyrillic).
    IsoIec8859_5 = 5,
    /// ISO/IEC 8859-7 (Greek).
    IsoIec8859_7 = 7,
    /// ISO/IEC 8859-9 (Turkish).
    IsoIec8859_9 = 9,
    /// ISO/IEC 8859-13 (Baltic Rim).
    IsoIec8859_13 = 13,
    /// ISO/IEC 8859-15 (Latin-9).
    IsoIec8859_15 = 15,
    /// ISO/IEC 8859-16 (South-Eastern European).
    IsoIec8859_16 = 16,
    /// KOI8-R (Russian).
    Koi8R = 80,
    /// KOI8-U (Ukrainian).
    Koi8U = 85,
    /// Invalid code page.
    Invalid = 255,
}

impl From<u8> for CodePage {
    fn from(value: u8) -> Self {
        match value {
            2 => CodePage::IsoIec8859_2,
            3 => CodePage::IsoIec8859_3,
            5 => CodePage::IsoIec8859_5,
            7 => CodePage::IsoIec8859_7,
            9 => CodePage::IsoIec8859_9,
            13 => CodePage::IsoIec8859_13,
            15 => CodePage::IsoIec8859_15,
            16 => CodePage::IsoIec8859_16,
            80 => CodePage::Koi8R,
            85 => CodePage::Koi8U,
            255 => CodePage::Invalid,
            _ => CodePage::IsoIec8859_1,
        }
    }
}
