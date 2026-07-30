use std::fmt;

use serde::Serialize;

use crate::{Export, Result, gen1, gen2};

/// # Top-Level Tachograph Data Container
/// Represents the parsed data from a tachograph file.
///
/// This enum is the main result of the parsing process. It categorizes the data
/// based on its source (Vehicle Unit or Card) and the tachograph generation
/// (Gen1 or Gen2), holding the corresponding detailed data structure.
/// It abstracts away the generation and data type (Vehicle Unit or Card),
/// allowing for unified handling of the parsed result.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum TachographData {
    /// Vehicle Unit data from a Gen1 tachograph.
    VUGen1(gen1::VUData),
    /// Vehicle Unit data from a Gen2 tachograph.
    VUGen2(gen2::VUData),
    /// Driver Card data from a Gen1 tachograph.
    CardGen1(gen1::CardData),
    /// Driver Card data from a Gen2 tachograph.
    CardGen2(gen2::CardData),
}

/// Typed envelope used by the WebAssembly boundary.
///
/// Native Rust and CLI exports retain the historical untagged representation.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "kind", content = "data", rename_all = "camelCase")]
pub enum WasmTachographData {
    #[serde(rename = "vuGen1")]
    VUGen1(gen1::VUData),
    #[serde(rename = "vuGen2")]
    VUGen2(gen2::VUData),
    CardGen1(gen1::CardData),
    CardGen2(gen2::CardData),
}

impl From<TachographData> for WasmTachographData {
    fn from(value: TachographData) -> Self {
        match value {
            TachographData::VUGen1(data) => Self::VUGen1(data),
            TachographData::VUGen2(data) => Self::VUGen2(data),
            TachographData::CardGen1(data) => Self::CardGen1(data),
            TachographData::CardGen2(data) => Self::CardGen2(data),
        }
    }
}

impl Export for TachographData {
    fn to_json(&self) -> Result<String>
    where
        Self: Serialize,
    {
        match self {
            TachographData::CardGen1(data) => data.to_json(),
            TachographData::CardGen2(data) => data.to_json(),
            TachographData::VUGen1(data) => data.to_json(),
            TachographData::VUGen2(data) => data.to_json(),
        }
    }

    fn to_xml(&self) -> Result<String>
    where
        Self: Serialize,
    {
        match self {
            TachographData::CardGen1(data) => data.to_xml(),
            TachographData::CardGen2(data) => data.to_xml(),
            TachographData::VUGen1(data) => data.to_xml(),
            TachographData::VUGen2(data) => data.to_xml(),
        }
    }
}

impl fmt::Display for TachographData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Uses the debug representation for a simple display format.
        write!(f, "{self:?}")
    }
}
