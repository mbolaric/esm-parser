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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_u16_to_string() {
        assert_eq!(time_u16_to_string(0), " 0: 0");
        assert_eq!(time_u16_to_string(59), " 0:59");
        assert_eq!(time_u16_to_string(60), " 1: 0");
        assert_eq!(time_u16_to_string(61), " 1: 1");
        assert_eq!(time_u16_to_string(1439), "23:59"); // 23 * 60 + 59
        assert_eq!(time_u16_to_string(1440), "24: 0");
    }

    #[test]
    fn test_u8_to_bool_valid() {
        assert!(!u8_to_bool(0).unwrap());
        assert!(u8_to_bool(1).unwrap());
    }

    #[test]
    fn test_u8_to_bool_invalid() {
        let result = u8_to_bool(2);
        assert!(result.is_err());
        match result.err().unwrap() {
            Error::InvalidInputByteValue(val) => assert_eq!(val, 2),
        }

        let result = u8_to_bool(255);
        assert!(result.is_err());
        match result.err().unwrap() {
            Error::InvalidInputByteValue(val) => assert_eq!(val, 255),
        }
    }
}
