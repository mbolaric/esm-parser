use std::fmt;

use serde::Serialize;

use crate::{Result, gen1, gen2};

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
pub enum SerializedTachographData {
    #[serde(rename = "vuGen1")]
    VUGen1(gen1::VUData),
    #[serde(rename = "vuGen2")]
    VUGen2(gen2::VUData),
    CardGen1(gen1::CardData),
    CardGen2(gen2::CardData),
}

impl From<TachographData> for SerializedTachographData {
    fn from(value: TachographData) -> Self {
        match value {
            TachographData::VUGen1(data) => Self::VUGen1(data),
            TachographData::VUGen2(data) => Self::VUGen2(data),
            TachographData::CardGen1(data) => Self::CardGen1(data),
            TachographData::CardGen2(data) => Self::CardGen2(data),
        }
    }
}

impl TachographData {
    /// Verifies the digital signatures of the parsed tachograph data.
    ///
    /// - For Gen1 Cards and Gen1 Vehicle Units, `erca_gen1_pk` is used (144 bytes).
    /// - For Gen2 Cards and Gen2 Vehicle Units, `erca_gen2_pk` is used (205 bytes).
    /// - For Combined Cards, both `erca_gen1_pk` and `erca_gen2_pk` are used.
    pub fn verify(
        &self,
        erca_gen1_pk: Option<&[u8]>,
        erca_gen2_pk: Option<&[u8]>,
    ) -> Result<crate::verification::TachographVerifyResult> {
        crate::verification::verify_tachograph_data(self, erca_gen1_pk, erca_gen2_pk)
    }

    /// Verifies digital signatures by reading ERCA keys from the given file paths.
    pub fn verify_with_erca_paths(
        &self,
        erca_gen1_path: Option<&str>,
        erca_gen2_path: Option<&str>,
    ) -> Result<crate::verification::TachographVerifyResult> {
        crate::verification::verify_tachograph_data_with_erca_paths(self, erca_gen1_path, erca_gen2_path)
    }
}

impl fmt::Display for TachographData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Uses the debug representation for a simple display format.
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Export;
    use crate::tacho::{TachographDataGeneration, TachographDataType, TachographHeader};

    #[test]
    fn blanket_export_preserves_untagged_tachograph_data() {
        let data = TachographData::CardGen1(gen1::CardData {
            header: TachographHeader {
                generation: TachographDataGeneration::FirstGeneration,
                data_type: TachographDataType::Card,
                card_in_vu_data: false,
            },
            card_data_responses: gen1::CardResponseParameterData::Unsupported,
        });

        let (expected_json, expected_xml) = match &data {
            TachographData::CardGen1(card) => (serde_json::to_string(card).unwrap(), quick_xml::se::to_string(card).unwrap()),
            _ => unreachable!(),
        };

        assert_eq!(data.to_json().unwrap(), expected_json);
        assert_eq!(data.to_xml().unwrap(), expected_xml);
    }
}
