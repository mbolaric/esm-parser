//! Signature verification for tachograph files.
//!
//! This module provides functionality to verify the digital signatures of tachograph data files.
//! It supports one Gen1 or Gen2 tachograph-card application, or one VU Overview download block,
//! at a time, dispatching to the appropriate verification logic after validating the corresponding
//! European Root Certification Authority (ERCA) certificate size.

use std::io::Read;

use binary_data::{BinReader, BinSeek};
use serde::Serialize;

use crate::tacho::{CardFilesMap, CardGeneration, DataFiles, VUFilesList, VUVerifyResult, VerifyResult, VuVerifyResult};
use crate::{Error, Export, Result, TachographData, gen1, gen2};

/// Unified verification result across any tachograph document (Card or VU, Gen1 or Gen2).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum TachographVerifyResult {
    Card(VerifyResult),
    CombinedCard(VerifyResult, VerifyResult),
    Vu(VUVerifyResult),
}

impl Export for TachographVerifyResult {}

/// Verifies the signature of tachograph card data files.
///
/// This function orchestrates the verification process by checking the inputs and delegating
/// to the appropriate generation-specific verification function (`gen1::verify` or `gen2::verify`).
/// The selection of the verification function is based on the length of the ERCA public key.
///
/// # Arguments
///
/// * `generation` - The card generation (`Gen1` or `Gen2`), used to validate the ERCA public key length.
///   A `Combined` card must be split into its Gen1 and Gen2 application maps and verified separately.
/// * `data_files` - A map containing the file data to be verified, with `CardFileID` as keys.
/// * `erca_pk` - A byte slice representing the European Root Certification Authority (ERCA) public key.
///   - For `Gen1`, this must be 144 bytes.
///   - For `Gen2`, this must be 205 bytes.
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
/// * `generation` is `Combined` (`Error::VerifyError`), because one file map and one ERCA certificate
///   cannot verify both applications.
/// * The length of `erca_pk` does not match the expected length for the specified `generation` (`Error::VerifyError`).
/// * The length of `erca_pk` is not a supported size (144 for Gen1, 205 for Gen2) (`Error::VerifyError`).
pub fn verify_card(generation: &CardGeneration, data_files: &CardFilesMap, erca_pk: &[u8]) -> Result<VerifyResult> {
    verify_card_with_time(generation, data_files, erca_pk, None)
}

/// Verifies the signature of tachograph card data files with an optional explicit validation timestamp.
pub fn verify_card_with_time(
    generation: &CardGeneration,
    data_files: &CardFilesMap,
    erca_pk: &[u8],
    validation_time: Option<u32>,
) -> Result<VerifyResult> {
    if matches!(generation, CardGeneration::Combined) {
        return Err(Error::VerifyError(
            "Combined card verification requires separate Gen1 and Gen2 application data maps.".to_owned(),
        ));
    }
    if data_files.is_empty() {
        return Err(Error::EmptyInputData("Data for verification are not provided.".to_owned()));
    }
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("ERCA Public Key are not provided.".to_owned()));
    }
    if erca_pk.len() != 144 && erca_pk.len() != 205 {
        return Err(Error::VerifyError(format!(
            "ERCA Public Key size of: {}, are not supported (Gen1 = 144 bytes, Gen2 = 205 bytes).",
            erca_pk.len()
        )));
    }

    if *generation == CardGeneration::Gen1 && erca_pk.len() != 144 {
        return Err(Error::VerifyError(format!(
            "ERCA Public Key for Card Gen1 need to be 144 bytes but is: {}, this are not supported (Gen1 = 144 bytes, Gen2 = 205 bytes).",
            erca_pk.len()
        )));
    }
    if *generation == CardGeneration::Gen2 && erca_pk.len() != 205 {
        return Err(Error::VerifyError(format!(
            "ERCA Public Key for Card Gen2 need to be 205 bytes but is: {}, this are not supported (Gen1 = 144 bytes, Gen2 = 205 bytes).",
            erca_pk.len()
        )));
    }

    if matches!(generation, CardGeneration::Gen1) {
        return gen1::verify(data_files, erca_pk.try_into().unwrap());
    }
    gen2::verify_with_time(data_files, erca_pk.try_into().unwrap(), validation_time)
}

/// Verifies both Gen1 and Gen2 applications of a Combined card.
pub fn verify_combined_card(
    gen1_data_files: &CardFilesMap,
    gen1_erca_pk: &[u8],
    gen2_data_files: &CardFilesMap,
    gen2_erca_pk: &[u8],
) -> Result<(VerifyResult, VerifyResult)> {
    let gen1_res = verify_card(&CardGeneration::Gen1, gen1_data_files, gen1_erca_pk)?;
    let gen2_res = verify_card(&CardGeneration::Gen2, gen2_data_files, gen2_erca_pk)?;
    Ok((gen1_res, gen2_res))
}

/// Verifies signatures by loading the ERCA public key from a file path.
///
/// This is a convenience function that reads the ERCA public key from the specified file path
/// and then calls the main `verify_card` function to perform the signature verification.
///
/// # Arguments
///
/// * `generation` - The card generation (`Gen1` or `Gen2`).
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
/// * Any of the conditions for an error in the `verify_card` function are met.
pub fn verify_card_with_erca_path(
    generation: CardGeneration,
    data_files: &CardFilesMap,
    erca_pk_file_path: &str,
) -> Result<VerifyResult> {
    let mut file = BinReader::open(erca_pk_file_path)?;
    let mut erca_pk = Vec::<u8>::with_capacity(file.len()?);
    file.read_to_end(&mut erca_pk)?;
    verify_card(&generation, data_files, &erca_pk)
}

/// Verifies the digital signatures of Vehicle Unit (VU) data files across all downloaded records.
pub fn verify_vu_full(data_files: &VUFilesList, erca_pk: &[u8]) -> Result<VUVerifyResult> {
    verify_vu_full_with_time(data_files, erca_pk, None)
}

/// Verifies the digital signatures of Vehicle Unit (VU) data files with an optional explicit validation timestamp.
pub fn verify_vu_full_with_time(
    data_files: &VUFilesList,
    erca_pk: &[u8],
    validation_time: Option<u32>,
) -> Result<VUVerifyResult> {
    if data_files.is_empty() {
        return Err(Error::EmptyInputData("Data for verification are not provided.".to_owned()));
    }
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("ERCA Public Key are not provided.".to_owned()));
    }
    match erca_pk.len() {
        144 => gen1::verify_vu_with_time(data_files, erca_pk.try_into().unwrap(), validation_time),
        205 => gen2::verify_vu_with_time(data_files, erca_pk.try_into().unwrap(), validation_time),
        other => Err(Error::VerifyError(format!(
            "ERCA Public Key size of: {other} bytes is not supported (Gen1 VU = 144 bytes, Gen2 VU = 205 bytes)."
        ))),
    }
}

/// Verifies full VU signatures by loading the ERCA public key from a file path.
pub fn verify_vu_full_with_erca_path(data_files: &VUFilesList, erca_pk_file_path: &str) -> Result<VUVerifyResult> {
    let mut file = BinReader::open(erca_pk_file_path)?;
    let mut erca_pk = Vec::<u8>::with_capacity(file.len()?);
    file.read_to_end(&mut erca_pk)?;
    verify_vu_full(data_files, &erca_pk)
}

/// Verifies the digital signatures of parsed tachograph data across any generation and document type.
///
/// * For Gen1 Cards and Gen1 Vehicle Units, `erca_gen1_pk` is used.
/// * For Gen2 Cards and Gen2 Vehicle Units, `erca_gen2_pk` is used.
/// * For Combined Cards, both keys are used.
pub fn verify_tachograph_data(
    data: &TachographData,
    erca_gen1_pk: Option<&[u8]>,
    erca_gen2_pk: Option<&[u8]>,
) -> Result<TachographVerifyResult> {
    match data {
        TachographData::CardGen1(card_gen1) => {
            let key = erca_gen1_pk.ok_or_else(|| Error::EmptyInputData("Gen1 ERCA key required for CardGen1".to_string()))?;
            let card_type: Option<&dyn DataFiles> = match &card_gen1.card_data_responses {
                crate::gen1::CardResponseParameterData::DriverCard(b) => Some(b.as_ref()),
                crate::gen1::CardResponseParameterData::WorkshopCard(b) => Some(b.as_ref()),
                crate::gen1::CardResponseParameterData::ControlCard(b) => Some(b.as_ref()),
                crate::gen1::CardResponseParameterData::CompanyCard(b) => Some(b.as_ref()),
                _ => None,
            };
            let card =
                card_type.ok_or_else(|| Error::VerifyError("Unsupported Card Type verification is not possible.".to_string()))?;
            let res = verify_card(&CardGeneration::Gen1, card.get_data_files(), key)?;
            Ok(TachographVerifyResult::Card(res))
        }
        TachographData::CardGen2(card_gen2) => match &card_gen2.card_data_responses {
            crate::gen2::CardResponseParameterData::DriverCard(crate::gen2::ParsedCard::Gen2(card)) => {
                let key = erca_gen2_pk.ok_or_else(|| Error::EmptyInputData("Gen2 ERCA key required for CardGen2".to_string()))?;
                let res = verify_card(&CardGeneration::Gen2, card.get_data_files(), key)?;
                Ok(TachographVerifyResult::Card(res))
            }
            crate::gen2::CardResponseParameterData::WorkshopCard(crate::gen2::ParsedCard::Gen2(card)) => {
                let key = erca_gen2_pk.ok_or_else(|| Error::EmptyInputData("Gen2 ERCA key required for CardGen2".to_string()))?;
                let res = verify_card(&CardGeneration::Gen2, card.get_data_files(), key)?;
                Ok(TachographVerifyResult::Card(res))
            }
            crate::gen2::CardResponseParameterData::DriverCard(crate::gen2::ParsedCard::Combined(gen1_card, gen2_card)) => {
                let key1 =
                    erca_gen1_pk.ok_or_else(|| Error::EmptyInputData("Gen1 ERCA key required for Combined card".to_string()))?;
                let key2 =
                    erca_gen2_pk.ok_or_else(|| Error::EmptyInputData("Gen2 ERCA key required for Combined card".to_string()))?;
                let (res1, res2) = verify_combined_card(gen1_card.get_data_files(), key1, gen2_card.get_data_files(), key2)?;
                Ok(TachographVerifyResult::CombinedCard(res1, res2))
            }
            crate::gen2::CardResponseParameterData::WorkshopCard(crate::gen2::ParsedCard::Combined(gen1_card, gen2_card)) => {
                let key1 =
                    erca_gen1_pk.ok_or_else(|| Error::EmptyInputData("Gen1 ERCA key required for Combined card".to_string()))?;
                let key2 =
                    erca_gen2_pk.ok_or_else(|| Error::EmptyInputData("Gen2 ERCA key required for Combined card".to_string()))?;
                let (res1, res2) = verify_combined_card(gen1_card.get_data_files(), key1, gen2_card.get_data_files(), key2)?;
                Ok(TachographVerifyResult::CombinedCard(res1, res2))
            }
            crate::gen2::CardResponseParameterData::CompanyCard(_) | crate::gen2::CardResponseParameterData::ControlCard(_) => {
                Err(Error::VerifyError("Gen2 signature verification is not applicable to Company and Control cards.".to_string()))
            }
            _ => Err(Error::VerifyError("Unsupported Gen2 Card Type verification is not possible.".to_string())),
        },
        TachographData::VUGen1(vu_gen1) => {
            let key = erca_gen1_pk.ok_or_else(|| Error::EmptyInputData("Gen1 ERCA key required for VUGen1".to_string()))?;
            let res = verify_vu_full(vu_gen1.get_data_files(), key)?;
            Ok(TachographVerifyResult::Vu(res))
        }
        TachographData::VUGen2(vu_gen2) => {
            let key = erca_gen2_pk.ok_or_else(|| Error::EmptyInputData("Gen2 ERCA key required for VUGen2".to_string()))?;
            let res = verify_vu_full(vu_gen2.get_data_files(), key)?;
            Ok(TachographVerifyResult::Vu(res))
        }
    }
}

/// Verifies signatures of parsed tachograph data by reading ERCA keys from file paths.
pub fn verify_tachograph_data_with_erca_paths(
    data: &TachographData,
    erca_gen1_file_path: Option<&str>,
    erca_gen2_file_path: Option<&str>,
) -> Result<TachographVerifyResult> {
    let mut erca_gen1_bytes = Vec::new();
    let erca_gen1_pk = if let Some(path) = erca_gen1_file_path {
        if !path.is_empty() {
            let mut file = BinReader::open(path)?;
            erca_gen1_bytes.reserve(file.len()?);
            file.read_to_end(&mut erca_gen1_bytes)?;
            Some(erca_gen1_bytes.as_slice())
        } else {
            None
        }
    } else {
        None
    };

    let mut erca_gen2_bytes = Vec::new();
    let erca_gen2_pk = if let Some(path) = erca_gen2_file_path {
        if !path.is_empty() {
            let mut file = BinReader::open(path)?;
            erca_gen2_bytes.reserve(file.len()?);
            file.read_to_end(&mut erca_gen2_bytes)?;
            Some(erca_gen2_bytes.as_slice())
        } else {
            None
        }
    } else {
        None
    };

    verify_tachograph_data(data, erca_gen1_pk, erca_gen2_pk)
}

/// A parsed VU Overview download block, tagged by generation.
pub enum VuOverview<'a> {
    Gen1(&'a gen1::VuOverview),
    Gen2(&'a gen2::VUOverview),
}

/// Verifies a downloaded VU's own certificate chain (ERCA -> MSCA -> VU).
///
/// This does not verify any downloaded VU data record (Activities, Events
/// and Faults, Speed, Technical Data) against that chain.
///
/// # Arguments
///
/// * `vu_overview` - The parsed VU Overview download block, tagged by generation.
/// * `erca_pk` - The ERCA public key for that same generation.
///   - For `Gen1`, this must be 144 bytes.
///   - For `Gen2`, this must be 205 bytes.
///
/// # Errors
///
/// This function can fail if:
/// * `erca_pk` is empty (`Error::EmptyInputData`).
/// * The length of `erca_pk` does not match the expected length for `vu_overview`'s generation.
/// * Any certificate in the chain fails to parse.
pub fn verify_vu(vu_overview: &VuOverview<'_>, erca_pk: &[u8]) -> Result<VuVerifyResult> {
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("ERCA Public Key are not provided.".to_owned()));
    }
    match vu_overview {
        VuOverview::Gen1(overview) => {
            if erca_pk.len() != 144 {
                return Err(Error::VerifyError(format!(
                    "ERCA Public Key size of: {}, is not supported for a Gen1 VU (Gen1 = 144 bytes).",
                    erca_pk.len()
                )));
            }
            gen1::vu::verify(overview, erca_pk.try_into().unwrap())
        }
        VuOverview::Gen2(overview) => {
            if erca_pk.len() != 205 {
                return Err(Error::VerifyError(format!(
                    "ERCA Public Key size of: {}, is not supported for a Gen2 VU (Gen2 = 205 bytes).",
                    erca_pk.len()
                )));
            }
            gen2::vu::verify(overview, erca_pk.try_into().unwrap())
        }
    }
}

/// Verifies VU Overview certificates by loading the ERCA public key from a file path.
pub fn verify_vu_with_erca_path(vu_overview: &VuOverview<'_>, erca_pk_file_path: &str) -> Result<VuVerifyResult> {
    let mut file = BinReader::open(erca_pk_file_path)?;
    let mut erca_pk = Vec::<u8>::with_capacity(file.len()?);
    file.read_to_end(&mut erca_pk)?;
    verify_vu(vu_overview, &erca_pk)
}

/// Verifies a VU's own certificate chain from its two raw certificate directly,
/// without requiring the full parsed VU Overview struct.
///
/// # Arguments
///
/// * `generation` - The VU generation (`Gen1` or `Gen2`). `Combined` is rejected.
/// * `member_state_certificate_raw` - The raw MSCA certificate bytes.
/// * `vu_certificate_raw` - The raw VU_Sign certificate bytes.
/// * `erca_pk` - The ERCA public key for that same generation.
///   - For `Gen1`, this must be 144 bytes.
///   - For `Gen2`, this must be 205 bytes.
///
/// # Errors
///
/// This function can fail if:
/// * `generation` is `Combined`.
/// * `erca_pk` is empty.
/// * The length of `erca_pk` does not match the expected length for generation.
/// * Any certificate in the chain fails to parse, fails its validity window
///   (Gen2 only), or fails to cryptographically verify.
pub fn verify_vu_certificate_chain(
    generation: &CardGeneration,
    member_state_certificate_raw: &[u8],
    vu_certificate_raw: &[u8],
    erca_pk: &[u8],
) -> Result<VuVerifyResult> {
    if matches!(generation, CardGeneration::Combined) {
        return Err(Error::VerifyError("A VU download is never a combined-generation document.".to_owned()));
    }
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("ERCA Public Key are not provided.".to_owned()));
    }
    match generation {
        CardGeneration::Gen1 => {
            if erca_pk.len() != 144 {
                return Err(Error::VerifyError(format!(
                    "ERCA Public Key size of: {}, is not supported for a Gen1 VU (Gen1 = 144 bytes).",
                    erca_pk.len()
                )));
            }
            crate::tachograph_gen1::vu_verification::verify_certificate_chain(
                member_state_certificate_raw,
                vu_certificate_raw,
                erca_pk.try_into().unwrap(),
            )
        }
        CardGeneration::Gen2 => {
            if erca_pk.len() != 205 {
                return Err(Error::VerifyError(format!(
                    "ERCA Public Key size of: {}, is not supported for a Gen2 VU (Gen2 = 205 bytes).",
                    erca_pk.len()
                )));
            }
            let validation_time = crate::tachograph_gen2::verification::current_unix_timestamp()?;
            crate::tachograph_gen2::vu_verification::verify_certificate_chain(
                member_state_certificate_raw,
                vu_certificate_raw,
                erca_pk.try_into().unwrap(),
                validation_time,
            )
        }
        CardGeneration::Combined => unreachable!("rejected above"),
    }
}
#[cfg(target_arch = "wasm32")]
mod wasm_support {
    use std::collections::HashMap;

    use serde::Serialize;
    use serde_wasm_bindgen::Serializer;
    use wasm_bindgen::prelude::*;

    use super::*;
    use crate::tacho::{CardFileData, CardFileID, VUFilesList};

    /// A WASM-bindgen wrapper for the `verify_card` function.
    ///
    /// This function exposes the signature verification functionality to JavaScript/WebAssembly environments.
    /// It handles the serialization and deserialization of data between JavaScript's `JsValue` and Rust's native types.
    ///
    /// # Arguments
    ///
    /// * `generation` - The card generation (`Gen1` or `Gen2`).
    /// * `data_files_map` - A `JsValue` representing the map of data files to be verified. This should be
    ///   an object where keys are `CardFileID` (as strings) and values are `CardFileData` (as byte arrays or similar).
    /// * `erca_pk` - A byte slice (`&[u8]`) containing the ERCA public key. Using a slice allows for an efficient, zero-copy transfer from JavaScript.
    ///
    /// # Returns
    ///
    /// A `Result` which, on success, contains a `JsValue` representing the serialized `VerifyResult`.
    /// On failure, it returns a `JsValue` containing the error message as a string.
    #[wasm_bindgen(js_name = verify_card, skip_typescript)]
    pub fn verify_card_wasm(
        generation: CardGeneration,
        data_files_map: JsValue,
        erca_pk: &[u8],
    ) -> std::result::Result<JsValue, JsValue> {
        let data_files: HashMap<CardFileID, CardFileData> = serde_wasm_bindgen::from_value(data_files_map)
            .map_err(|err| JsValue::from_str(&format!("Invalid input: {}", err)))?;

        let result = verify_card(&generation, &data_files, erca_pk);
        match result {
            Ok(data) => data.serialize(&Serializer::json_compatible()).map_err(|e| e.into()),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// A WASM-bindgen wrapper for full VU signature verification.
    #[wasm_bindgen(js_name = verify_vu_full, skip_typescript)]
    pub fn verify_vu_full_wasm(data_files: JsValue, erca_pk: &[u8]) -> std::result::Result<JsValue, JsValue> {
        let files: VUFilesList =
            serde_wasm_bindgen::from_value(data_files).map_err(|err| JsValue::from_str(&format!("Invalid input: {}", err)))?;

        let result = verify_vu_full(&files, erca_pk);
        match result {
            Ok(data) => data.serialize(&Serializer::json_compatible()).map_err(|e| e.into()),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Backwards-compatible WASM-bindgen wrapper for VU signature verification.
    #[wasm_bindgen(js_name = verify_vu, skip_typescript)]
    pub fn verify_vu_wasm(data_files: JsValue, erca_pk: &[u8]) -> std::result::Result<JsValue, JsValue> {
        verify_vu_full_wasm(data_files, erca_pk)
    }
}
