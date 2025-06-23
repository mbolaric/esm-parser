use std::collections::HashMap;

use binary_data::{BigEndian, BinSeek, ReadBytes};
use log::debug;

use crate::tacho::{CardFileID, CardItem, TachographHeader};
use crate::{Readable, Result};

pub enum CardGeneration {
    Gen1,
    Gen2,
    Combined,
}

#[derive(Clone)]
pub struct CardDataFile {
    pub card_file_id: CardFileID,
    pub appendix: u8,
    pub card_file_notes: String,
    pub size: u32,
    pub signature: Option<Vec<u8>>,
    pub data: Option<Vec<u8>>,
}

impl Readable<CardDataFile> for CardDataFile {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<CardDataFile> {
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

        Ok(Self {
            card_file_id,
            appendix,
            card_file_notes,
            size,
            signature: None,
            data,
        })
    }
}

pub trait Card<D> {
    fn get_header(&self) -> &TachographHeader;
    fn get_data(&self) -> &Vec<D>;
}

impl<D> dyn Card<D> {
    fn procces_card_data_file(
        current_card_item: CardItem<CardDataFile>,
        card_items: &mut HashMap<CardFileID, CardItem<CardDataFile>>,
    ) -> &str {
        // FIXME:
        card_items.insert(current_card_item.card_file_id.clone(), current_card_item);
        ""
    }

    pub fn from_data<R: ReadBytes + BinSeek>(
        reader: &mut R,
        parse_card: &(dyn Fn(CardFileID, &CardDataFile) -> Result<D>),
    ) -> Result<HashMap<CardFileID, CardItem<D>>> {
        let mut card_data_files: HashMap<CardFileID, CardItem<CardDataFile>> = HashMap::new();
        let mut card_items: HashMap<CardFileID, CardItem<D>> = HashMap::new();
        let mut card_notes: String = "".to_owned();

        while reader.pos()? < reader.len()? {
            let data_file = CardDataFile::read(reader)?;
            debug!("Card::from_data - {:?}", data_file.card_file_id.clone());

            let current_card_item = CardItem {
                card_file_id: data_file.card_file_id.clone(),
                data: data_file.clone(),
            };

            card_notes.push_str(<dyn Card<D>>::procces_card_data_file(
                current_card_item,
                &mut card_data_files,
            ));
        }

        // FIXME: Reurn on object CardData ...
        //let data = parse_card(&card_data_files, card_notes)?;

        Ok(card_items)
    }
}
