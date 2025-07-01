use binary_data::{BinFile, BinReader, BinSeek};
use log::debug;

use crate::{
    Error, Result, TachographData, gen1, gen2,
    tacho::{TachographDataGeneration, TachographDataType, TachographHeader},
};

#[derive(Debug)]
pub struct EsmParser<'a> {
    esm_file_path: &'a str,
    data: Option<TachographData>,
}

impl<'a> EsmParser<'a> {
    fn new(esm_file_path: &'a str) -> Self {
        EsmParser { esm_file_path, data: None }
    }

    fn read_by_data_type(&mut self, header: TachographHeader, data: &mut BinFile) -> Result<TachographData> {
        debug!("EsmParser::read_by_data_type - Type: {:?}, Generation: {:?}", header.data_type, header.generation);
        match header.data_type {
            TachographDataType::VU => match header.generation {
                TachographDataGeneration::FirstGeneration => Ok(TachographData::VUGen1(gen1::VUData::from_data(header, data)?)),
                TachographDataGeneration::SecondGeneration => Ok(TachographData::VUGen2(gen2::VUData::from_data(header, data)?)),
                _ => Err(Error::InvalidDataGeneration),
            },
            TachographDataType::Card => {
                if header.card_in_vu_data {
                    data.skip_n_bytes::<2>()?
                }
                match header.generation {
                    TachographDataGeneration::FirstGeneration => {
                        Ok(TachographData::CardGen1(gen1::CardData::from_data(header, data)?))
                    }
                    TachographDataGeneration::SecondGeneration => Err(Error::NotImplemented),
                    _ => Err(Error::InvalidDataGeneration),
                }
            }
        }
    }

    fn parse_inner(&mut self) -> Result<()> {
        let mut file = BinReader::open(self.esm_file_path)?;
        debug!("EsmParser::parse_inner - File: {:?}", file);

        let header = TachographHeader::from_data(&file.read_n_bytes::<2>()?, file.metadata().len())?;
        file.seek(0)?;

        let vu_data = self.read_by_data_type(header, &mut file)?;
        self.data = Some(vu_data);
        Ok(())
    }

    pub fn parse(esm_file_path: &'a str) -> Result<EsmParser<'a>> {
        let mut parser = EsmParser::new(esm_file_path);
        parser.parse_inner()?;
        Ok(parser)
    }

    pub fn get_data(&self) -> Option<&TachographData> {
        self.data.as_ref()
    }
}
