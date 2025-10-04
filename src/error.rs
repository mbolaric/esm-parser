use crate::common;
use core::fmt;

/// Represents an error that can occur during parsing.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred.
    File(std::io::Error),
    /// A binary data error occurred.
    Binary(binary_data::Error),
    /// The header length is invalid.
    InvalidHeaderLength,
    /// The header data is invalid.
    InvalidHeaderData,
    /// An error occurred during data generation.
    InvalidDataGeneration,
    /// An error occurred during data parsing.
    InvalidDataParse(String),
    /// An error occurred during data encoding.
    InvalidDataEncode(String),
    /// A duplicate card file was detected.
    DuplicateCardFile,
    /// A signature was found before a card file.
    SignatureBeforeCardFile,
    /// A partial card file was detected.
    PartialCardFile,
    /// The card type is not supported.
    UnsupportedCardType,
    /// The card type is unknown.
    UnknownCardType,
    /// The feature is not yet implemented.
    NotImplemented,
    /// A required card file is missing.
    MissingCardFile(String),
    /// The card type is not supported.
    NotSupportedCardType(String),
    /// The driving licence number is corrupted.
    CorruptedDrivingLicenceNumber,
    /// An error occurred when decoding an unknown card type.
    UnknownCardTypeDecoding,
    /// A record is out of range.
    RecordOutOfRange(String),
    /// An error occurred with a card activity daily record.
    CardActivityDailyRecord(String),
    /// An error occurred when we try to export data with serde.
    Export(String),
    /// Input data into function are empty.
    EmptyInputData(String),
    VerifyError(String),
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

impl From<common::string_encoding::Error> for Error {
    fn from(value: common::string_encoding::Error) -> Self {
        Error::InvalidDataParse(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Export(value.to_string())
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(value: serde_wasm_bindgen::Error) -> Self {
        Error::Export(value.to_string())
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Error::Export(value.to_string())
    }
}

impl From<quick_xml::de::DeError> for Error {
    fn from(value: quick_xml::de::DeError) -> Self {
        Error::Export(value.to_string())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
