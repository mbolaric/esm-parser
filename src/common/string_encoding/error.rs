use core::fmt;

/// Defines the possible errors that can occur during string decoding.
#[derive(Debug)]
pub enum Error {
    /// Error indicating that a character is not a valid IA5 (ASCII) character.
    InvalidIA5CharacterNotASCII,
    /// Error indicating that the input is too short for a fixed-length IA5 string.
    InputTooShortForIA5String,
    /// Error indicating that a string is not a valid IA5 (ASCII) string.
    InvalidIA5StringNotASCII,
}

/// Implements the `Display` trait for the `Error` enum to provide a user-friendly error message.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
