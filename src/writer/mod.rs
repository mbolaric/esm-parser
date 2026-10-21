//! Opt-in binary serialization for supported tachograph values and records.
//!
//! This is not a complete Card/VU `.DDD` file writer and does not expose a
//! JavaScript writer API. Signature policies apply only to implementations that
//! explicitly consume them; current record writers do not sign or replace signatures.

mod common;
mod tachograph;

pub use common::{
    SignaturePolicy, SignatureSigner, Validatable, Writable, WritableWithParams, WriteOptions, encode_code_page_padded,
    encode_ia5_padded, ia5_fix_string_to_bytes, string_to_bytes, try_encode_byte, try_string_to_bytes, validate_bcd,
    validate_code_page_string, validate_fixed_bytes, validate_ia5_string, write_code_page_string, write_ia5_string,
    write_records,
};
