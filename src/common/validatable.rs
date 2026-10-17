use crate::{BCDString, CodePage, Error, Result, encode_code_page_padded, encode_ia5_padded};

/// A trait for types that can validate their internal consistency and specification compliance
/// before serialization or storage.
///
/// Types implementing `Validatable` ensure that their fields adhere to tachograph
/// regulations (EU 1360/2002 Annex 1B and EU 2016/799 Annex 1C Appendix 1 Data Dictionary).
/// If any field contains invalid, unencodable, or out-of-range values, `validate()` returns
/// an error, preventing corrupted or invalid data from ever being serialized.
pub trait Validatable {
    /// Validates the internal consistency and range of all fields.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the data is valid and safe to serialize.
    /// * `Err(Error)` if any validation rule is violated.
    fn validate(&self) -> Result<()>;
}

/// Validates that a string can be losslessly encoded in `code_page` and fits within `N` bytes.
pub fn validate_code_page_string<const N: usize>(field: &str, s: &str, code_page: &CodePage) -> Result<()> {
    encode_code_page_padded::<N>(field, s, code_page).map(|_| ())
}

/// Validates that a string contains only IA5 (ASCII) characters and fits within `N` bytes.
pub fn validate_ia5_string<const N: usize>(field: &str, s: &str) -> Result<()> {
    encode_ia5_padded::<N>(field, s).map(|_| ())
}

/// Validates that a byte slice has the exact expected length `N`.
pub fn validate_fixed_bytes<const N: usize>(field: &str, bytes: &[u8]) -> Result<()> {
    if bytes.len() != N {
        return Err(Error::ValidationError(format!("{field} must be {N} bytes, got {}", bytes.len())));
    }
    Ok(())
}

/// Validates that a string is a valid fixed-length BCD string of `2 * BYTES` ASCII decimal digits.
pub fn validate_bcd<const BYTES: usize>(field: &str, s: &str) -> Result<[u8; BYTES]> {
    BCDString::encode_strict::<BYTES>(s).map_err(|_| {
        Error::ValidationError(format!("{field} must be {} ASCII decimal digits, got '{s}'", BYTES * 2))
    })
}

/// Blanket implementation of `Validatable` for slices of validatable elements.
impl<T: Validatable> Validatable for [T] {
    fn validate(&self) -> Result<()> {
        for item in self {
            item.validate()?;
        }
        Ok(())
    }
}

/// Blanket implementation of `Validatable` for vectors of validatable elements.
impl<T: Validatable> Validatable for Vec<T> {
    fn validate(&self) -> Result<()> {
        self.as_slice().validate()
    }
}

/// Blanket implementation of `Validatable` for `Option<T>` where `T: Validatable`.
impl<T: Validatable> Validatable for Option<T> {
    fn validate(&self) -> Result<()> {
        if let Some(item) = self {
            item.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ValidItem(bool);

    impl Validatable for ValidItem {
        fn validate(&self) -> Result<()> {
            if self.0 { Ok(()) } else { Err(Error::ValidationError("Item is invalid".into())) }
        }
    }

    #[test]
    fn test_validatable_slice_and_vec() {
        let valid = vec![ValidItem(true), ValidItem(true)];
        assert!(valid.validate().is_ok());

        let invalid = vec![ValidItem(true), ValidItem(false)];
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_validatable_option() {
        let none_item: Option<ValidItem> = None;
        assert!(none_item.validate().is_ok());

        let some_valid: Option<ValidItem> = Some(ValidItem(true));
        assert!(some_valid.validate().is_ok());

        let some_invalid: Option<ValidItem> = Some(ValidItem(false));
        assert!(some_invalid.validate().is_err());
    }
}
