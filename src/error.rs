use core::fmt;

#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    Binary(binary_data::Error),
    InvalidHeaderLength,
    InvalidHeaderData,
    InvalidDataGeneration,
    InvalidDataParse(String),
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::File(value)
    }
}

impl From<binary_data::Error> for Error {
    fn from(value: binary_data::Error) -> Self {
        Error::Binary(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error::InvalidDataParse(value.to_string())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
