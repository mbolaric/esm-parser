use binary_data::{BinMemoryBuffer, BinReader, BinSeek, ReadBytes};
use log::debug;

use crate::{
    Error, Result, TachographData, gen1, gen2,
    tacho::{TachographDataGeneration, TachographDataType, TachographHeader},
};

/// Reads the tachograph data based on the data type and generation specified in the header.
///
/// # Arguments
///
/// * `header` - The `TachographHeader` containing metadata about the data.
/// * `reader` - A mutable reference to the reader to read the data from.
///
/// # Returns
///
/// A `Result` containing the parsed `TachographData` or an `Error` if parsing fails.
fn read_by_data_type<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<TachographData> {
    debug!("EsmParser::read_by_data_type - Type: {:?}, Generation: {:?}", header.data_type, header.generation);
    match header.data_type {
        TachographDataType::VU => match header.generation {
            TachographDataGeneration::FirstGeneration => Ok(TachographData::VUGen1(gen1::VUData::from_data(header, reader)?)),
            TachographDataGeneration::SecondGeneration => Ok(TachographData::VUGen2(gen2::VUData::from_data(header, reader)?)),
            _ => Err(Error::InvalidDataGeneration),
        },
        TachographDataType::Card => {
            if header.card_in_vu_data {
                // We skip 2 bytes
                let _ = reader.read_bytes::<2>();
            }
            match header.generation {
                TachographDataGeneration::FirstGeneration => {
                    Ok(TachographData::CardGen1(gen1::CardData::from_data(header, reader)?))
                }
                TachographDataGeneration::SecondGeneration => {
                    Ok(TachographData::CardGen2(gen2::CardData::from_data(header, reader)?))
                }
                _ => Err(Error::InvalidDataGeneration),
            }
        }
    }
}

/// Parses the tachograph data from a reader after reading the header.
///
/// # Arguments
///
/// * `header_data` - A byte slice containing the first 2 bytes of the file, used to determine the data type and generation.
/// * `data_len` - The total length of the data.
/// * `reader` - A mutable reference to the reader to read the data from.
///
/// # Returns
///
/// A `Result` containing the parsed `TachographData` or an `Error` if parsing fails.
fn parse_inner<R: ReadBytes + BinSeek>(header_data: &[u8; 2], data_len: u64, reader: &mut R) -> Result<TachographData> {
    let header = TachographHeader::from_data(header_data, data_len)?;
    reader.seek(0)?;

    read_by_data_type(header, reader)
}

/// This methods provides the entry point for parsing DDD files. It automatically
/// detects the generation of the tachograph data (Gen1 or Gen2) and the type of
/// data (Vehicle Unit or Driver Card) and parses it accordingly.
///
/// # Arguments
///
/// * `esm_file_path` - The path to the DDD file.
///
/// # Returns
///
/// A `Result` containing the parsed `TachographData` or an `Error` if parsing fails.
pub fn parse_from_file(esm_file_path: &str) -> Result<TachographData> {
    let mut file = BinReader::open(esm_file_path)?;
    debug!("EsmParser::parse_inner - File: {file:?}");

    parse_inner(&file.read_n_bytes::<2>()?, file.metadata().len(), &mut file)
}

/// This methods provides the entry point for parsing DDD files. It automatically
/// detects the generation of the tachograph data (Gen1 or Gen2) and the type of
/// data (Vehicle Unit or Driver Card) and parses it accordingly.
///
/// # Arguments
///
/// * `esm_data` - The binary data from DDD file.
///
/// # Returns
///
/// A `Result` containing the parsed `TachographData` or an `Error` if parsing fails.
pub fn parse_from_memory(esm_data: &[u8]) -> Result<TachographData> {
    let mut reader = BinMemoryBuffer::from(esm_data);
    debug!("EsmParser::parse_inner - File: {reader:?}");

    parse_inner(&reader.read_bytes::<2>()?, reader.len()? as u64, &mut reader)
}

#[cfg(target_arch = "wasm32")]
mod wasm_support {
    use super::*;
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::prelude::*;

    /// WebAssembly-specific wrapper for the `parse_from_memory` function.
    ///
    /// This function is exposed to JavaScript as `parse_from_memory`. It takes a byte slice
    /// of DDD file data, calls the native Rust parser, and converts the result into a
    /// JavaScript-compatible format (`JsValue`).
    ///
    /// # Arguments
    ///
    /// * `esm_data` - A byte slice (`&[u8]`) containing the raw binary data from a DDD file.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<JsValue, JsValue>`, which corresponds to a JavaScript
    /// `Promise`.
    /// - On success, the `Promise` resolves with a `JsValue` object representing the parsed `TachographData`.
    /// - On failure, the `Promise` rejects with a `JsValue` containing the error message.
    #[wasm_bindgen(js_name = parse_from_memory)]
    pub fn parse_from_memory_wasm(esm_data: &[u8]) -> std::result::Result<JsValue, JsValue> {
        let result = parse_from_memory(esm_data);
        match result {
            Ok(data) => to_value(&data).map_err(|e| e.into()),
            Err(e) => Err(to_value(&e.to_string()).unwrap_or(JsValue::NULL)),
        }
    }
}
