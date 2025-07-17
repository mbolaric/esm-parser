#[derive(Debug, PartialEq)]
pub enum CodePage {
    IsoIec8859_1 = 1,
    IsoIec8859_2 = 2,
    IsoIec8859_3 = 3,
    IsoIec8859_5 = 5,
    IsoIec8859_7 = 7,
    IsoIec8859_9 = 9,
    IsoIec8859_13 = 13,
    IsoIec8859_15 = 15,
    IsoIec8859_16 = 16,
    Koi8R = 80,
    Koi8U = 85,
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
