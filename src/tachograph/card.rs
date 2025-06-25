use std::collections::HashMap;

use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use log::debug;

use crate::tacho::{CardFileID, CardItem, TachographHeader};
use crate::{Error, Readable, Result};

pub enum CardGeneration {
    Gen1,
    Gen2,
    Combined,
}

#[derive(Debug, Clone)]
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
    pub fn get_mem_reader(
        card_file_id: &CardFileID,
        data: &HashMap<CardFileID, CardItem<CardDataFile>>,
    ) -> Result<BinMemoryBuffer> {
        let reader: Option<BinMemoryBuffer> =
            data.get(card_file_id)
                .and_then(|card_item: &CardItem<CardDataFile>| {
                    card_item
                        .data
                        .data
                        .as_ref()
                        .and_then(|bin_data| Some(BinMemoryBuffer::from(bin_data.clone())))
                });
        if reader.is_some() {
            return Ok(reader.unwrap());
        }
        Err(Error::MissingCardFile(card_file_id.to_string()))
    }

    fn procces_card_data_file(
        current_card_item: CardItem<CardDataFile>,
        card_items: &mut HashMap<CardFileID, CardItem<CardDataFile>>,
    ) -> Result<String> {
        let mut data_file = &current_card_item.data;
        let mut card_file_notes = "".to_owned();
        match current_card_item.card_file_id {
            CardFileID::ICC
            | CardFileID::IC
            | CardFileID::Tachograph
            | CardFileID::ApplicationIdentification
            | CardFileID::EventsData
            | CardFileID::FaultsData
            | CardFileID::DriverActivityData
            | CardFileID::VehiclesUsed
            | CardFileID::Places
            | CardFileID::CurrentUsage
            | CardFileID::ControlActivityData
            | CardFileID::Calibration
            | CardFileID::SensorInstallationData
            | CardFileID::ControllerActivityData
            | CardFileID::CardDownload
            | CardFileID::Identification
            | CardFileID::DrivingLicenseInfo
            | CardFileID::SpecificConditions
            | CardFileID::MF
            | CardFileID::CardCertificate
            | CardFileID::CACertificate => {
                let card_file_temp = card_items.get_mut(&current_card_item.card_file_id);
                if data_file.appendix == 0 {
                    if card_file_temp.is_some() {
                        return Err(Error::DuplicateCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes = format!(
                            "[{}] {}",
                            &current_card_item.card_file_id, &data_file.card_file_notes
                        );
                    }
                } else {
                    // Signature
                    if card_file_temp.is_none() {
                        return Err(Error::SignatureBeforeCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes = format!(
                            "[{} (signature)] {}",
                            &current_card_item.card_file_id, &data_file.card_file_notes
                        );
                    }
                    card_file_temp.unwrap().data.signature = data_file.data.clone()
                }
            }
            _ => return Err(Error::UnknownCardType),
        }

        card_items.insert(current_card_item.card_file_id.clone(), current_card_item);
        Ok(card_file_notes)
    }

    pub fn from_data<R: ReadBytes + BinSeek>(
        reader: &mut R,
        parse_card: &(dyn Fn(&HashMap<CardFileID, CardItem<CardDataFile>>, &String) -> Result<D>),
    ) -> Result<D> {
        let mut card_data_files: HashMap<CardFileID, CardItem<CardDataFile>> = HashMap::new();
        let mut card_notes: String = "".to_owned();

        while reader.pos()? < reader.len()? {
            let data_file = CardDataFile::read(reader)?;
            debug!("Card::from_data - {:?}", data_file.card_file_id.clone());

            let current_card_item = CardItem {
                card_file_id: data_file.card_file_id.clone(),
                data: data_file.clone(),
            };

            let mut temp_notes =
                <dyn Card<D>>::procces_card_data_file(current_card_item, &mut card_data_files)?;
            if !temp_notes.is_empty() {
                temp_notes = format!("{}\r\n", temp_notes);
            }

            card_notes.push_str(temp_notes.as_str());
        }

        // Card Data is Partial
        if reader.pos()? != reader.len()? {
            return Err(Error::PartialCardFile);
        }

        let data = parse_card(&card_data_files, &card_notes)?;
        Ok(data)
    }
}
