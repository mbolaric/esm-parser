use std::fmt::Display;

use crate::{BCDString, Readable};

#[derive(Debug)]
pub struct StringDate {
    pub year: String,
    pub month: String,
    pub day: String,
}

impl Display for StringDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Readable<StringDate> for StringDate {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<StringDate> {
        let year = BCDString::decode(&reader.read_into_vec(2)?)?;
        let month = BCDString::decode(&reader.read_into_vec(1)?)?;
        let day = BCDString::decode(&reader.read_into_vec(1)?)?;

        Ok(Self { year, month, day })
    }
}
