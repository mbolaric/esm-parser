use core::fmt;
use std::collections::HashMap;

use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::tacho::{ApplicationIdentification, CardChipIdentification, CardFileID, CardIccIdentification, TachographHeader};
use crate::{Error, Readable, Result};

pub type CardParseFunc<D> = dyn Fn(&CardFilesDataByCardGeneration) -> Result<D>;
pub type CardFilesMap = HashMap<CardFileID, CardFileData>;

pub trait DataFiles {
    fn get_data_files(&self) -> &CardFilesMap;
}

/// Returns a card's files sorted by `CardFileID`, so parsing does not depend on `HashMap`'s iteration order.
pub fn sorted_card_files(card_data_files: &CardFilesMap) -> Vec<(&CardFileID, &CardFileData)> {
    let mut items: Vec<_> = card_data_files.iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    items
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "ParsedCardGeneration"))]
pub enum CardGeneration {
    Gen1,
    Gen2,
    Combined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardFileSection {
    Gen1Data,
    Gen1Signature,
    Gen2Data,
    Gen2Signature,
}

impl TryFrom<u8> for CardFileSection {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Gen1Data),
            1 => Ok(Self::Gen1Signature),
            2 => Ok(Self::Gen2Data),
            3 => Ok(Self::Gen2Signature),
            _ => Err(Error::InvalidCardFileAppendix(value)),
        }
    }
}

impl fmt::Display for CardGeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CardGeneration::Combined => "Combined",
            CardGeneration::Gen1 => "Gen1",
            CardGeneration::Gen2 => "Gen2",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct CardFileData {
    #[serde(rename = "cardFileId")]
    pub card_file_id: CardFileID,
    pub appendix: u8,
    #[serde(rename = "cardFileNotes")]
    pub card_file_notes: String,
    pub size: u32,
    pub signature: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

impl CardFileData {
    fn vector_into_reader(&self, data: &Option<Vec<u8>>) -> Result<BinMemoryBuffer> {
        let reader = data.as_ref().map(|bin_data| BinMemoryBuffer::from(bin_data.clone()));
        if let Some(mem_reader) = reader {
            return Ok(mem_reader);
        }
        Err(Error::MissingCardFile(self.card_file_id.to_string()))
    }

    pub fn data_into_reader(&self) -> Result<BinMemoryBuffer> {
        self.vector_into_reader(&self.data)
    }

    pub fn signature_into_reader(&self) -> Result<BinMemoryBuffer> {
        self.vector_into_reader(&self.signature)
    }

    pub fn data_len(&self) -> usize {
        if let Some(data) = &self.data { data.len() } else { 0 }
    }
}

impl Readable for CardFileData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardFileData> {
        let card_file_id: CardFileID = reader.read_u16::<BigEndian>()?.into();
        let appendix = reader.read_u8()?;
        let size = reader.read_u16::<BigEndian>()? as u32;

        let mut card_file_notes: String = "".to_owned();
        let data = if reader.pos()? == reader.len()? {
            card_file_notes = "Missing card file data after header".to_owned();
            None
        } else {
            Some(reader.read_into_vec(size)?)
        };

        Ok(Self { card_file_id, appendix, card_file_notes, size, signature: None, data })
    }
}

#[derive(Debug, Clone)]
pub struct CardFilesDataByCardGenerationItem {
    pub card_files_data: CardFilesMap,
    pub card_notes: String,
}

impl CardFilesDataByCardGenerationItem {
    pub fn new() -> Self {
        Self { card_files_data: HashMap::new(), card_notes: "".to_owned() }
    }

    pub fn is_empty(&self) -> bool {
        self.card_files_data.is_empty()
    }
}

impl Default for CardFilesDataByCardGenerationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CardFilesDataByCardGeneration {
    pub card_files_data_gen1: CardFilesDataByCardGenerationItem,
    pub card_files_data_gen2: CardFilesDataByCardGenerationItem,
}

impl CardFilesDataByCardGeneration {
    pub fn new() -> Self {
        Self {
            card_files_data_gen1: CardFilesDataByCardGenerationItem::default(),
            card_files_data_gen2: CardFilesDataByCardGenerationItem::default(),
        }
    }

    pub fn get_card_generation(&self) -> Result<CardGeneration> {
        match (self.card_files_data_gen1.is_empty(), self.card_files_data_gen2.is_empty()) {
            (false, false) => Ok(CardGeneration::Combined),
            (false, true) => Ok(CardGeneration::Gen1),
            (true, false) => Ok(CardGeneration::Gen2),
            (true, true) => Err(Error::EmptyCardFiles),
        }
    }
}

impl Default for CardFilesDataByCardGeneration {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Card<D> {
    fn get_header(&self) -> &TachographHeader;
    fn get_data(&self) -> &Vec<D>;
}

impl<D> dyn Card<D> {
    pub fn get_mem_reader(card_file_id: &CardFileID, data: &CardFilesMap) -> Result<BinMemoryBuffer> {
        let reader: Option<BinMemoryBuffer> = data
            .get(card_file_id)
            .and_then(|card_item: &CardFileData| card_item.data.as_ref().map(|bin_data| BinMemoryBuffer::from(bin_data.clone())));
        if let Some(mem_reader) = reader {
            return Ok(mem_reader);
        }
        Err(Error::MissingCardFile(card_file_id.to_string()))
    }

    pub fn parse_ic(card_data_files: &CardFilesMap) -> Result<CardChipIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::IC, card_data_files)?;
        let card_chip_identification = CardChipIdentification::read(&mut reader)?;
        Ok(card_chip_identification)
    }

    pub fn parse_icc(card_data_files: &CardFilesMap) -> Result<CardIccIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ICC, card_data_files)?;
        let card_icc_identification = CardIccIdentification::read(&mut reader)?;
        Ok(card_icc_identification)
    }

    pub fn parse_card_application_identification<T: Readable>(card_data_files: &CardFilesMap) -> Result<T> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ApplicationIdentification, card_data_files)?;
        let application_identification = T::read(&mut reader)?;
        Ok(application_identification)
    }

    pub fn parse_application_identification(card_data_files: &CardFilesMap) -> Result<ApplicationIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ApplicationIdentification, card_data_files)?;
        let application_identification = ApplicationIdentification::read(&mut reader)?;
        Ok(application_identification)
    }

    pub fn parse_by_card_file_id<T: Readable>(card_file_id: &CardFileID, card_data_files: &CardFilesMap) -> Result<T> {
        let mut reader = <dyn Card<D>>::get_mem_reader(card_file_id, card_data_files)?;
        let result = T::read(&mut reader)?;
        Ok(result)
    }

    pub fn parse_optional_by_card_file_id<T: Readable>(
        card_file_id: &CardFileID,
        card_data_files: &CardFilesMap,
    ) -> Result<Option<T>> {
        card_data_files
            .get(card_file_id)
            .map(|card_file| {
                let mut reader = card_file.data_into_reader()?;
                T::read(&mut reader)
            })
            .transpose()
    }

    fn process_card_data_file(data_file: CardFileData, card_items: &mut CardFilesDataByCardGeneration) -> Result<()> {
        if data_file.card_file_id == CardFileID::Unknown {
            debug!("Card::process_card_data_file - CardDataFile: {data_file:?}");
            return Err(Error::UnknownCardType);
        }

        let section = CardFileSection::try_from(data_file.appendix)?;
        debug!("Card::process_card_data_file - CardFileID: {:?}, Section: {:?}", data_file.card_file_id, section);

        let generation_files = match section {
            CardFileSection::Gen1Data | CardFileSection::Gen1Signature => &mut card_items.card_files_data_gen1,
            CardFileSection::Gen2Data | CardFileSection::Gen2Signature => &mut card_items.card_files_data_gen2,
        };
        let card_file_id = data_file.card_file_id.clone();

        match section {
            CardFileSection::Gen1Data | CardFileSection::Gen2Data => {
                if generation_files.card_files_data.contains_key(&card_file_id) {
                    return Err(Error::DuplicateCardFile);
                }
                if !data_file.card_file_notes.is_empty() {
                    generation_files.card_notes.push_str(&format!("[{}] {}", data_file.card_file_id, data_file.card_file_notes));
                }
                generation_files.card_files_data.insert(card_file_id, data_file);
            }
            CardFileSection::Gen1Signature | CardFileSection::Gen2Signature => {
                let card_file = generation_files.card_files_data.get_mut(&card_file_id).ok_or(Error::SignatureBeforeCardFile)?;
                if !data_file.card_file_notes.is_empty() {
                    generation_files
                        .card_notes
                        .push_str(&format!("[{} (signature)] {}", data_file.card_file_id, data_file.card_file_notes));
                }
                card_file.signature = data_file.data;
            }
        }

        Ok(())
    }

    pub fn from_data<R: ReadBytes + BinSeek>(reader: &mut R, parse_card: &CardParseFunc<D>) -> Result<D> {
        let mut card_data_files = CardFilesDataByCardGeneration::new();

        while reader.pos()? < reader.len()? {
            let current_data_file = CardFileData::read(reader)?;
            debug!("Card::from_data - {:?}, Length : {:?}", current_data_file.card_file_id.clone(), current_data_file.data_len());
            <dyn Card<D>>::process_card_data_file(current_data_file, &mut card_data_files)?;
        }

        // Card Data is Partial
        if reader.pos()? != reader.len()? {
            return Err(Error::PartialCardFile);
        }

        let data = parse_card(&card_data_files)?;
        Ok(data)
    }
}

pub trait CardParser: Sized {
    fn parse(card_data_files: &HashMap<CardFileID, CardFileData>, card_notes: &str) -> Result<Box<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tacho::TimeReal;

    fn card_file(data: Vec<u8>) -> CardFileData {
        CardFileData {
            card_file_id: CardFileID::CardDownload,
            appendix: 0,
            card_file_notes: String::new(),
            size: data.len() as u32,
            signature: None,
            data: Some(data),
        }
    }

    #[test]
    fn optional_card_file_returns_none_when_absent() {
        let card_files = CardFilesMap::new();

        let value = <dyn Card<()>>::parse_optional_by_card_file_id::<TimeReal>(&CardFileID::CardDownload, &card_files)
            .expect("an absent optional file should not fail");

        assert!(value.is_none());
    }

    #[test]
    fn optional_card_file_propagates_malformed_data() {
        let mut card_files = CardFilesMap::new();
        card_files.insert(CardFileID::CardDownload, card_file(vec![0; 3]));

        let result = <dyn Card<()>>::parse_optional_by_card_file_id::<TimeReal>(&CardFileID::CardDownload, &card_files);

        assert!(result.is_err());
    }

    #[test]
    fn optional_card_file_parses_present_data() {
        let mut card_files = CardFilesMap::new();
        card_files.insert(CardFileID::CardDownload, card_file(42_u32.to_be_bytes().to_vec()));

        let value = <dyn Card<()>>::parse_optional_by_card_file_id::<TimeReal>(&CardFileID::CardDownload, &card_files)
            .expect("valid optional data should parse")
            .expect("the optional file is present");

        assert_eq!(value.get_data(), 42);
    }

    #[test]
    fn rejects_unknown_card_file_appendix() {
        let mut card_files = CardFilesDataByCardGeneration::new();
        let mut file = card_file(Vec::new());
        file.appendix = 4;

        let error = <dyn Card<()>>::process_card_data_file(file, &mut card_files).unwrap_err();

        assert!(matches!(error, Error::InvalidCardFileAppendix(4)));
    }

    #[test]
    fn routes_card_data_and_signatures_by_appendix() {
        let mut card_files = CardFilesDataByCardGeneration::new();

        let mut gen1_data = card_file(vec![1]);
        gen1_data.appendix = 0;
        let mut gen1_signature = card_file(vec![2]);
        gen1_signature.appendix = 1;
        let mut gen2_data = card_file(vec![3]);
        gen2_data.appendix = 2;
        let mut gen2_signature = card_file(vec![4]);
        gen2_signature.appendix = 3;

        <dyn Card<()>>::process_card_data_file(gen1_data, &mut card_files).unwrap();
        <dyn Card<()>>::process_card_data_file(gen1_signature, &mut card_files).unwrap();
        <dyn Card<()>>::process_card_data_file(gen2_data, &mut card_files).unwrap();
        <dyn Card<()>>::process_card_data_file(gen2_signature, &mut card_files).unwrap();

        assert_eq!(card_files.card_files_data_gen1.card_files_data[&CardFileID::CardDownload].signature, Some(vec![2]));
        assert_eq!(card_files.card_files_data_gen2.card_files_data[&CardFileID::CardDownload].signature, Some(vec![4]));
    }

    #[test]
    fn card_generation_requires_at_least_one_file() {
        let card_files = CardFilesDataByCardGeneration::new();

        assert!(matches!(card_files.get_card_generation(), Err(Error::EmptyCardFiles)));
    }

    #[test]
    fn card_generation_matches_available_file_sets() {
        let mut gen1_only = CardFilesDataByCardGeneration::new();
        gen1_only.card_files_data_gen1.card_files_data.insert(CardFileID::CardDownload, card_file(Vec::new()));
        assert_eq!(gen1_only.get_card_generation().unwrap(), CardGeneration::Gen1);

        let mut gen2_only = CardFilesDataByCardGeneration::new();
        gen2_only.card_files_data_gen2.card_files_data.insert(CardFileID::CardDownload, card_file(Vec::new()));
        assert_eq!(gen2_only.get_card_generation().unwrap(), CardGeneration::Gen2);

        gen1_only.card_files_data_gen2.card_files_data.insert(CardFileID::CardDownload, card_file(Vec::new()));
        assert_eq!(gen1_only.get_card_generation().unwrap(), CardGeneration::Combined);
    }
}
