use std::collections::HashMap;

use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use log::debug;

use crate::tacho::{ApplicationIdentification, CardChipIdentification, CardFileID, CardIccIdentification, TachographHeader};
use crate::{Error, Readable, Result};

pub type CardParseFunc<D> = (dyn Fn(&CardDataFilesByCardGeneration) -> Result<D>);

#[derive(Debug, Clone, PartialEq)]
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

    pub fn data_len(&self) -> usize {
        if let Some(data) = &self.data { data.len() } else { 0 }
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

#[derive(Debug, Clone)]
pub struct CardDataFilesByCardGenerationItem {
    pub card_data_files: HashMap<CardFileID, CardDataFile>,
    pub card_notes: String,
}

impl CardDataFilesByCardGenerationItem {
    pub fn new() -> Self {
        Self { card_data_files: HashMap::new(), card_notes: "".to_owned() }
    }

    pub fn is_empty(&self) -> bool {
        self.card_data_files.is_empty()
    }
}

impl Default for CardDataFilesByCardGenerationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CardDataFilesByCardGeneration {
    pub card_data_files_gen1: CardDataFilesByCardGenerationItem,
    pub card_data_files_gen2: CardDataFilesByCardGenerationItem,
}

impl CardDataFilesByCardGeneration {
    pub fn new() -> Self {
        Self {
            card_data_files_gen1: CardDataFilesByCardGenerationItem::default(),
            card_data_files_gen2: CardDataFilesByCardGenerationItem::default(),
        }
    }

    pub fn get_card_generation(&self) -> CardGeneration {
        if !self.card_data_files_gen1.is_empty() && !self.card_data_files_gen2.is_empty() {
            return CardGeneration::Combined;
        }
        if !self.card_data_files_gen1.is_empty() {
            return CardGeneration::Gen1;
        }
        CardGeneration::Gen2
    }
}

impl Default for CardDataFilesByCardGeneration {
    fn default() -> Self {
        Self::new()
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

    pub fn parse_ic(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardChipIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::IC, card_data_files)?;
        let card_chip_identification = CardChipIdentification::read(&mut reader)?;
        Ok(card_chip_identification)
    }

    pub fn parse_icc(card_data_files: &HashMap<CardFileID, CardDataFile>) -> Result<CardIccIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ICC, card_data_files)?;
        let card_icc_identification = CardIccIdentification::read(&mut reader)?;
        Ok(card_icc_identification)
    }

    pub fn parse_card_application_identification<T: Readable<T>>(
        card_data_files: &HashMap<CardFileID, CardDataFile>,
    ) -> Result<T> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ApplicationIdentification, card_data_files)?;
        let application_identification = T::read(&mut reader)?;
        Ok(application_identification)
    }

    pub fn parse_application_identification(
        card_data_files: &HashMap<CardFileID, CardDataFile>,
    ) -> Result<ApplicationIdentification> {
        let mut reader = <dyn Card<D>>::get_mem_reader(&CardFileID::ApplicationIdentification, card_data_files)?;
        let application_identification = ApplicationIdentification::read(&mut reader)?;
        Ok(application_identification)
    }

    fn procces_card_data_file(data_file: CardDataFile, card_items: &mut CardDataFilesByCardGeneration) -> Result<()> {
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
                debug!(
                    "Card::procces_card_data_file - CardFileID: {:?}, Appendix: {:?}",
                    data_file.card_file_id, data_file.appendix
                );

                let (card_file_temp, card_file_notes) = if data_file.appendix == 0 || data_file.appendix == 1 {
                    (
                        card_items.card_data_files_gen1.card_data_files.get_mut(&data_file.card_file_id),
                        &mut card_items.card_data_files_gen1.card_notes,
                    )
                } else {
                    (
                        card_items.card_data_files_gen2.card_data_files.get_mut(&data_file.card_file_id),
                        &mut card_items.card_data_files_gen2.card_notes,
                    )
                };

                if data_file.appendix == 0 || data_file.appendix == 2 {
                    if card_file_temp.is_some() {
                        return Err(Error::DuplicateCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes.push_str(&format!("[{}] {}", &data_file.card_file_id, &data_file.card_file_notes));
                    }
                    if data_file.appendix == 0 {
                        card_items.card_data_files_gen1.card_data_files.insert(data_file.card_file_id.clone(), data_file);
                    } else {
                        card_items.card_data_files_gen2.card_data_files.insert(data_file.card_file_id.clone(), data_file);
                    }
                } else {
                    // Signature
                    if card_file_temp.is_none() {
                        return Err(Error::SignatureBeforeCardFile);
                    }
                    if !data_file.card_file_notes.is_empty() {
                        card_file_notes
                            .push_str(&format!("[{} (signature)] {}", &data_file.card_file_id, &data_file.card_file_notes));
                    }
                    card_file_temp.unwrap().signature = data_file.data.clone()
                }
            }
            CardFileID::VehicleUnitsUsed
            | CardFileID::GnssPlaces
            | CardFileID::CardSignCertificate
            | CardFileID::LinkCertificate => {
                debug!(
                    "Card::procces_card_data_file - CardFileID: {:?}, Appendix: {:?} is ignored for now.",
                    data_file.card_file_id, data_file.appendix
                );
            }
            _ => {
                debug!("Card::procces_card_data_file - CardFileID: {:?}", data_file.card_file_id);
                return Err(Error::UnknownCardType);
            }
        }

        Ok(())
    }

    pub fn from_data<R: ReadBytes + BinSeek>(reader: &mut R, parse_card: &CardParseFunc<D>) -> Result<D> {
        let mut card_data_files = CardDataFilesByCardGeneration::new();

        while reader.pos()? < reader.len()? {
            let current_data_file = CardDataFile::read(reader)?;
            debug!("Card::from_data - {:?}, Length : {:?}", current_data_file.card_file_id.clone(), current_data_file.data_len());
            <dyn Card<D>>::procces_card_data_file(current_data_file, &mut card_data_files)?;
        }

        // Card Data is Partial
        if reader.pos()? != reader.len()? {
            return Err(Error::PartialCardFile);
        }

        let data = parse_card(&card_data_files)?;
        Ok(data)
    }
}
