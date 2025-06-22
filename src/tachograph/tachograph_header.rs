use crate::{error::{Error, Result}, tacho::{TachographDataGeneration, TachographDataType}, CARD_HEADER, CARD_HEADER_VU_DATA, MINIMUM_G2_CARD_DATA_LENGTH, VU_HEADER_G1, VU_HEADER_G2, VU_HEADER_G2_V2};

#[derive(Debug)]
pub struct TachographHeader {
    pub generation: TachographDataGeneration,
    pub data_type: TachographDataType,
    pub card_in_vu_data: bool
}

impl TachographHeader {
    pub fn from_data(data: &[u8], data_length: u64) -> Result<TachographHeader> {
        if data.len() != 2 {
            return Err(Error::InvalidHeaderLength);
        }

        TachographHeader::parse_header(data, data_length)
    }

    fn parse_header(header: &[u8], data_length: u64) -> Result<TachographHeader> {
        if header.eq(&VU_HEADER_G1) {
            return Ok(TachographHeader {
                generation: TachographDataGeneration::FirstGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false
            });
        }

        if header.eq(&VU_HEADER_G2) || header.eq(&VU_HEADER_G2_V2) {
            return Ok(TachographHeader {
                generation: TachographDataGeneration::SecondGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false
            });
        }

        if header.eq(&CARD_HEADER) {
            let generation: TachographDataGeneration = if data_length >= MINIMUM_G2_CARD_DATA_LENGTH { TachographDataGeneration::SecondGeneration } else { TachographDataGeneration::FirstGeneration };
            return Ok(TachographHeader {
                generation,
                data_type: TachographDataType::Card,
                card_in_vu_data: false
            });            
        }

        if header.eq(&CARD_HEADER_VU_DATA) {
            let generation: TachographDataGeneration = if data_length >= MINIMUM_G2_CARD_DATA_LENGTH { TachographDataGeneration::SecondGeneration } else { TachographDataGeneration::FirstGeneration };
            return Ok(TachographHeader {
                generation,
                data_type: TachographDataType::Card,
                card_in_vu_data: true
            });            
        }
        
        Err(Error::InvalidHeaderData)
    }
}