use binary_data::{BinMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::error::{Error, Result};
use crate::tacho::{CardFileID, TachographDataGeneration, TachographDataType};
use crate::{CARD_HEADER_VU_DATA, VU_HEADER_G1, VU_HEADER_G2, VU_HEADER_G2_V2};

/// Represents the header of a tachograph file.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
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
    /// Detects type and generation from complete download bytes.
    ///
    /// VUs use their overview TREP identifier. Cards use their section appendices,
    /// including Gen2 sections in combined cards. File size is never used to infer generation.
    pub fn from_data(data: &[u8]) -> Result<Self> {
        Self::from_reader(&mut BinMemoryBuffer::from(data))
    }

    /// Detects the download header from the start of a seekable reader.
    /// Restores the original reader position on success and on detection failure.
    pub fn from_reader<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
        let original_position = reader.pos()?;
        let result = Self::detect(reader);
        reader.seek(original_position)?;
        result
    }

    fn detect<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
        reader.seek(0)?;
        let length = reader.len()?;
        if length < 2 {
            return Err(Error::InvalidHeaderLength);
        }
        let prefix = reader.read_bytes::<2>()?;
        match prefix {
            VU_HEADER_G1 => Ok(Self {
                generation: TachographDataGeneration::FirstGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false,
            }),
            VU_HEADER_G2 | VU_HEADER_G2_V2 => Ok(Self {
                generation: TachographDataGeneration::SecondGeneration,
                data_type: TachographDataType::VU,
                card_in_vu_data: false,
            }),
            prefix if prefix == CARD_HEADER_VU_DATA || CardFileID::from(u16::from_be_bytes(prefix)) != CardFileID::Unknown => {
                let card_in_vu_data = prefix == CARD_HEADER_VU_DATA;
                reader.seek(if card_in_vu_data { 2 } else { 0 })?;
                if reader.pos()? == length {
                    return Err(Error::PartialCardFile);
                }
                let mut generation = TachographDataGeneration::FirstGeneration;
                while reader.pos()? < length {
                    if length - reader.pos()? < 5 {
                        return Err(Error::PartialCardFile);
                    }
                    let section = reader.read_bytes::<5>()?;
                    if CardFileID::from(u16::from_be_bytes([section[0], section[1]])) == CardFileID::Unknown {
                        return Err(Error::UnknownCardType);
                    }
                    match section[2] {
                        0 | 1 => (),
                        2 | 3 => generation = TachographDataGeneration::SecondGeneration,
                        appendix => return Err(Error::InvalidCardFileAppendix(appendix)),
                    }
                    let size = usize::from(u16::from_be_bytes([section[3], section[4]]));
                    let position = reader.pos()?;
                    if size > length - position {
                        return Err(Error::PartialCardFile);
                    }
                    reader.seek(position + size)?;
                }
                Ok(Self { generation, data_type: TachographDataType::Card, card_in_vu_data })
            }
            _ => Err(Error::InvalidHeaderData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section(appendix: u8, payload: &[u8]) -> Vec<u8> {
        let mut bytes = vec![0, 2, appendix];
        bytes.extend(u16::try_from(payload.len()).unwrap().to_be_bytes());
        bytes.extend(payload);
        bytes
    }

    #[test]
    fn detects_cards_from_section_boundaries_instead_of_size() {
        let large_gen1 = section(0, &vec![0x02; 40_000]);
        let small_gen2 = section(2, &[0]);
        let gen1_with_embedded_markers = section(0, &[0, 2, 2, 0, 0, 0, 2, 3, 0, 0]);
        let combined = [section(0, &[0]), section(2, &[0]), section(1, &[0])].concat();
        for (bytes, expected) in [
            (large_gen1, TachographDataGeneration::FirstGeneration),
            (small_gen2, TachographDataGeneration::SecondGeneration),
            (gen1_with_embedded_markers, TachographDataGeneration::FirstGeneration),
            (combined, TachographDataGeneration::SecondGeneration),
        ] {
            for wrapped in [false, true] {
                let data = if wrapped { [CARD_HEADER_VU_DATA.to_vec(), bytes.clone()].concat() } else { bytes.clone() };
                let header = TachographHeader::from_data(&data).unwrap();
                assert_eq!(header.generation, expected);
                assert_eq!(header.data_type, TachographDataType::Card);
                assert_eq!(header.card_in_vu_data, wrapped);
            }
        }
    }

    #[test]
    fn detects_vu_generations_from_overview_trep() {
        for (prefix, generation) in [
            (VU_HEADER_G1, TachographDataGeneration::FirstGeneration),
            (VU_HEADER_G2, TachographDataGeneration::SecondGeneration),
            (VU_HEADER_G2_V2, TachographDataGeneration::SecondGeneration),
        ] {
            let header = TachographHeader::from_data(&prefix).unwrap();
            assert_eq!(header.generation, generation);
            assert_eq!(header.data_type, TachographDataType::VU);
            assert!(!header.card_in_vu_data);
        }
    }

    #[test]
    fn cards_can_start_with_any_known_ef() {
        for id in [CardFileID::IC, CardFileID::ApplicationIdentification, CardFileID::CardCertificate, CardFileID::Identification]
        {
            for appendix in [0, 2] {
                let mut bytes = section(appendix, &[0]);
                bytes[..2].copy_from_slice(&(id as u16).to_be_bytes());
                for wrapped in [false, true] {
                    let bytes = if wrapped { [CARD_HEADER_VU_DATA.to_vec(), bytes.clone()].concat() } else { bytes.clone() };
                    let header = TachographHeader::from_data(&bytes).unwrap();
                    assert_eq!(header.data_type, TachographDataType::Card);
                    assert_eq!(header.card_in_vu_data, wrapped);
                    assert_eq!(
                        header.generation,
                        if appendix == 2 {
                            TachographDataGeneration::SecondGeneration
                        } else {
                            TachographDataGeneration::FirstGeneration
                        }
                    );
                }
            }
        }
    }

    #[test]
    fn rejects_truncated_sections_invalid_appendices_and_unknown_headers() {
        for data in [Vec::new(), vec![0]] {
            assert!(matches!(TachographHeader::from_data(&data), Err(Error::InvalidHeaderLength)));
        }
        assert!(matches!(TachographHeader::from_data(&[0xFF, 0xFF]), Err(Error::InvalidHeaderData)));
        for data in [
            vec![0, 2],
            vec![0, 2, 0, 0],
            vec![0, 2, 0, 0, 2, 1],
            vec![0x76, 6, 0, 2],
            vec![0x76, 6],
            vec![5, 1],
            vec![5, 1, 0, 0, 2, 1],
        ] {
            assert!(matches!(TachographHeader::from_data(&data), Err(Error::PartialCardFile)));
        }
        let unknown = vec![0xFF, 0xFF, 0, 0, 0];
        for bytes in [[CARD_HEADER_VU_DATA.to_vec(), unknown.clone()].concat(), [section(0, &[]), unknown].concat()] {
            assert!(matches!(TachographHeader::from_data(&bytes), Err(Error::UnknownCardType)));
        }
        assert!(matches!(TachographHeader::from_data(&section(4, &[])), Err(Error::InvalidCardFileAppendix(4))));
    }

    #[test]
    fn reader_position_is_preserved_on_success_and_failure() {
        for (data, valid) in [(section(2, &[0]), true), (vec![5, 1, 0, 0, 0], true), (vec![5, 1, 0, 0, 2, 1], false)] {
            let mut reader = BinMemoryBuffer::from(data);
            reader.seek(3).unwrap();
            assert_eq!(TachographHeader::from_reader(&mut reader).is_ok(), valid);
            assert_eq!(reader.pos().unwrap(), 3);
        }
    }
}
