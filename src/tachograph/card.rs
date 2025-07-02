use std::collections::HashMap;

use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use log::debug;

use crate::tacho::{CardFileID, TachographHeader};
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

impl CardDataFile {
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
}

impl Readable<CardDataFile> for CardDataFile {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardDataFile> {
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

pub trait Card<D> {
    fn get_header(&self) -> &TachographHeader;
    fn get_data(&self) -> &Vec<D>;
}

impl<D> dyn Card<D> {
    pub fn get_mem_reader(card_file_id: &CardFileID, data: &HashMap<CardFileID, CardDataFile>) -> Result<BinMemoryBuffer> {
        let reader: Option<BinMemoryBuffer> = data
            .get(card_file_id)
            .and_then(|card_item: &CardDataFile| card_item.data.as_ref().map(|bin_data| BinMemoryBuffer::from(bin_data.clone())));
        if let Some(mem_reader) = reader {
            return Ok(mem_reader);
        }
        Err(Error::MissingCardFile(card_file_id.to_string()))
    }

    fn procces_card_data_file(data_file: CardDataFile, card_items: &mut HashMap<CardFileID, CardDataFile>) -> Result<String> {
        let mut card_file_notes = "".to_owned();
        match data_file.card_file_id {
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
                let card_file_temp = card_items.get_mut(&data_file.card_file_id);
                if data_file.appendix == 0 {
                    if card_file_temp.is_some() {
                        return Err(Error::DuplicateCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes = format!("[{}] {}", &data_file.card_file_id, &data_file.card_file_notes);
                    }
                    card_items.insert(data_file.card_file_id.clone(), data_file);
                } else {
                    // Signature
                    if card_file_temp.is_none() {
                        return Err(Error::SignatureBeforeCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes = format!("[{} (signature)] {}", &data_file.card_file_id, &data_file.card_file_notes);
                    }
                    card_file_temp.unwrap().signature = data_file.data.clone()
                }
            }
            _ => return Err(Error::UnknownCardType),
        }

        Ok(card_file_notes)
    }

    pub fn from_data<R: ReadBytes + BinSeek>(
        reader: &mut R,
        parse_card: &(dyn Fn(&HashMap<CardFileID, CardDataFile>, &String) -> Result<D>),
    ) -> Result<D> {
        let mut card_data_files: HashMap<CardFileID, CardDataFile> = HashMap::new();
        let mut card_notes: String = "".to_owned();

        while reader.pos()? < reader.len()? {
            let current_data_file = CardDataFile::read(reader)?;
            debug!("Card::from_data - {:?}", current_data_file.card_file_id.clone());

            let mut temp_notes = <dyn Card<D>>::procces_card_data_file(current_data_file, &mut card_data_files)?;
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
