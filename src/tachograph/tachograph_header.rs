use serde::Serialize;

use crate::{
    CARD_HEADER, CARD_HEADER_VU_DATA, MINIMUM_G2_CARD_DATA_LENGTH, VU_HEADER_G1, VU_HEADER_G2, VU_HEADER_G2_V2,
    error::{Error, Result},
    tacho::{TachographDataGeneration, TachographDataType},
};

/// Represents the header of a tachograph file.
#[derive(Debug, Clone, Serialize)]
pub struct TachographHeader {
    /// The generation of the tachograph data.
    pub generation: TachographDataGeneration,
    /// The type of the tachograph data.
    #[serde(rename = "dataType")]
    pub data_type: TachographDataType,
    /// Whether the card data is from a VU file.
    #[serde(rename = "cardInVuData")]
    pub card_in_vu_data: bool,
}

impl TachographHeader {
    /// Creates a new `TachographHeader` from the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The header data.
    /// * `data_length` - The length of the data.
    ///
    /// # Returns
    ///
    /// A new `TachographHeader` instance.
    pub fn from_data(data: &[u8], data_length: u64) -> Result<TachographHeader> {
        if data.len() != 2 {
            return Err(Error::InvalidHeaderLength);
        }

        TachographHeader::parse_header(data, data_length)
    }

    /// Parses the header data and returns a new `TachographHeader`.
    ///
    /// # Arguments
    ///
    /// * `header` - The header data.
    /// * `data_length` - The length of the data.
    ///
    /// # Returns
    ///
    /// A new `TachographHeader` instance.
    fn parse_header(header: &[u8], data_length: u64) -> Result<TachographHeader> {
        if header.eq(&VU_HEADER_G1) {
            return Ok(TachographHeader {
                generation: TachographDataGeneration::FirstGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false,
            });
        }

        if header.eq(&VU_HEADER_G2) || header.eq(&VU_HEADER_G2_V2) {
            return Ok(TachographHeader {
                generation: TachographDataGeneration::SecondGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false,
            });
        }

        if header.eq(&CARD_HEADER) {
            let generation: TachographDataGeneration = if data_length >= MINIMUM_G2_CARD_DATA_LENGTH {
                TachographDataGeneration::SecondGeneration
            } else {
                TachographDataGeneration::FirstGeneration
            };
            return Ok(TachographHeader { generation, data_type: TachographDataType::Card, card_in_vu_data: false });
        }

        if header.eq(&CARD_HEADER_VU_DATA) {
            let generation: TachographDataGeneration = if data_length >= MINIMUM_G2_CARD_DATA_LENGTH {
                TachographDataGeneration::SecondGeneration
            } else {
                TachographDataGeneration::FirstGeneration
            };
            return Ok(TachographHeader { generation, data_type: TachographDataType::Card, card_in_vu_data: true });
        }

        Err(Error::InvalidHeaderData)
    }
}
