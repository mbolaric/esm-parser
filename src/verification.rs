use std::collections::HashMap;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::{
    Error, Result,
    tacho::{CardFileData, CardFileID, VerifyResult},
};

pub fn verify(data_files: &HashMap<CardFileID, CardFileData>, erca_pk: &[u8]) -> Result<VerifyResult> {
    if data_files.is_empty() {
        return Err(Error::EmptyInputData("Data for verification are not provided.".to_owned()));
    }
    if erca_pk.is_empty() {
        return Err(Error::EmptyInputData("EC Public Key are not provided.".to_owned()));
    }
    if erca_pk.len() != 144 && erca_pk.len() != 205 {
        return Err(Error::EmptyInputData(format!(
            "EC Public Key size of {} are not correct (Gen1 = 144 bytes, Gen2 = 205 bytes).",
            erca_pk.len()
        )));
    }
    Err(Error::NotImplemented)
}

#[wasm_bindgen(js_name = verify)]
pub fn verify_wasm(data_files_map: JsValue, erca_pk: Vec<u8>) -> std::result::Result<JsValue, JsValue> {
    let data_files: HashMap<CardFileID, CardFileData> =
        serde_wasm_bindgen::from_value(data_files_map).map_err(|err| JsValue::from_str(&format!("Invalid input: {}", err)))?;

    let result = verify(&data_files, &erca_pk);
    match result {
        Ok(data) => to_value(&data).map_err(|e| e.into()),
        Err(e) => Err(to_value(&e.to_string()).unwrap_or(JsValue::NULL)),
    }
}
