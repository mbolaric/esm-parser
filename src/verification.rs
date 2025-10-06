//! Signature verification for tachograph files.
//!
//! This module provides functionality to verify the digital signatures of tachograph data files.
//! It supports both Gen1 and Gen2 tachograph data, dispatching to the appropriate verification
//! logic based on the provided European Root Certification Authority (ERCA) public key size.

use std::{collections::HashMap, io::Read};

use binary_data::{BinReader, BinSeek};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::{
    Error, Result, gen1, gen2,
    tacho::{CardFileData, CardFileID, CardFilesMap, VerifyResult},
};

/// Verifies the signature of tachograph data files.
///
/// This function orchestrates the verification process by checking the inputs and delegating
/// to the appropriate generation-specific verification function (`gen1::verify` or `gen2::verify`).
/// The selection is based on the length of the ERCA public key.
///
/// # Arguments
///
/// * `data_files` - A map containing the file data to be verified, with `CardFileID` as keys.
/// * `erca_pk` - A byte slice representing the European Root Certification Authority (ERCA) public key.
///
/// # Supported Formats
///
/// * **Gen1:** Detected when `erca_pk` length is 144 bytes.
/// * **Gen2:** Detected when `erca_pk` length is 205 bytes.
///
/// # Returns
///
/// A `Result` containing a `VerifyResult` on success, which details the outcome of the
/// signature verification for each file.
///
/// # Errors
///
/// This function will return an `Error` if:
/// * `data_files` is empty (`Error::EmptyInputData`).
/// * `erca_pk` is empty (`Error::EmptyInputData`).
/// * The length of `erca_pk` is not 144 (Gen1) or 205 (Gen2) bytes, indicating an unsupported key size (`Error::EmptyInputData`).
pub fn verify(data_files: &CardFilesMap, erca_pk: &[u8]) -> Result<VerifyResult> {
    if data_files.is_empty() {
        return Err(Error::EmptyInputData("Data for verification are not provided.".to_owned()));
    }
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("ERCA Public Key are not provided.".to_owned()));
    }
    if erca_pk.len() != 144 && erca_pk.len() != 205 {
        return Err(Error::EmptyInputData(format!(
            "ERCA Public Key size of: {}, are not supported (Gen1 = 144 bytes, Gen2 = 205 bytes).",
            erca_pk.len()
        )));
    }
    if erca_pk.len() == 144 {
        return gen1::verify(data_files, erca_pk.try_into().unwrap());
    }
    gen2::verify(data_files, erca_pk.try_into().unwrap())
}

/// Verifies signatures by loading the ERCA public key from a file path.
///
/// This is a convenience function that reads the ERCA public key from the specified file path
/// and then calls the main `verify` function to perform the signature verification.
///
/// # Arguments
///
/// * `data_files` - A map containing the file data to be verified.
/// * `erca_pk_file_path` - The file system path to the ERCA public key file.
///
/// # Returns
///
/// A `Result` containing a `VerifyResult` on success.
///
/// # Errors
///
/// This function can fail if:
/// * The file at `erca_pk_file_path` cannot be opened or read.
/// * Any of the conditions for an error in the `verify` function are met.
pub fn verify_with_erca_path(data_files: &CardFilesMap, erca_pk_file_path: &str) -> Result<VerifyResult> {
    let mut file = BinReader::open(erca_pk_file_path)?;
    let mut erca_pk = Vec::<u8>::with_capacity(file.len()?);
    file.read_to_end(&mut erca_pk)?;
    verify(data_files, &erca_pk)
}

/// A WASM-bindgen wrapper for the `verify` function.
///
/// This function exposes the signature verification functionality to JavaScript/WebAssembly environments.
/// It handles the serialization and deserialization of data between JavaScript's `JsValue` and Rust's native types.
///
/// # Arguments
///
/// * `data_files_map` - A `JsValue` representing the map of data files to be verified. This should be
///   an object where keys are `CardFileID` (as strings) and values are `CardFileData` (as byte arrays or similar).
/// * `erca_pk` - A byte slice (`&[u8]`) containing the ERCA public key. Using a slice allows for an efficient, zero-copy transfer from JavaScript.
///
/// # Returns
///
/// A `Result` which, on success, contains a `JsValue` representing the serialized `VerifyResult`.
/// On failure, it returns a `JsValue` containing the error message as a string.
#[wasm_bindgen(js_name = verify)]
pub fn verify_wasm(data_files_map: JsValue, erca_pk: &[u8]) -> std::result::Result<JsValue, JsValue> {
    let data_files: HashMap<CardFileID, CardFileData> =
        serde_wasm_bindgen::from_value(data_files_map).map_err(|err| JsValue::from_str(&format!("Invalid input: {}", err)))?;

    let result = verify(&data_files, erca_pk);
    match result {
        Ok(data) => to_value(&data).map_err(|e| e.into()),
        Err(e) => Err(to_value(&e.to_string()).unwrap_or(JsValue::NULL)),
    }
}
