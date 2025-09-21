use crate::{gen1, gen2};
use std::fmt;

use serde::Serialize;

/// # Top-Level Tachograph Data Container
/// Represents the parsed data from a tachograph file.
///
/// This enum is the main result of the parsing process. It categorizes the data
/// based on its source (Vehicle Unit or Card) and the tachograph generation
/// (Gen1 or Gen2), holding the corresponding detailed data structure.
/// It abstracts away the generation and data type (Vehicle Unit or Card),
/// allowing for unified handling of the parsed result.
#[derive(Debug, Serialize)]
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

impl fmt::Display for TachographData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Uses the debug representation for a simple display format.
        write!(f, "{self:?}")
    }
}
