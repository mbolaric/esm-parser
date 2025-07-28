use core::fmt;

#[allow(dead_code)]
pub fn time_u16_to_string(time_min: u16) -> String {
    let hours: u16 = time_min / 60;
    let mins: u16 = time_min % 60;
    format!("{hours:2}:{mins:2}")
}

pub fn u8_to_bool(value: u8) -> Result<bool, Error> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::InvalidInputByteValue(value)),
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidInputByteValue(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidInputByteValue(v) => write!(f, "{self:?} - {v:?}"),
        }
    }
}
