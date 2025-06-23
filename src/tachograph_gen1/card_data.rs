use std::collections::HashMap;

use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::CardResponseParameterData;
use crate::tacho::{self, CardDataFile, CardFileID, CardItem, TachographHeader};
use crate::Result;

#[derive(Debug)]
pub struct CardData {
    header: TachographHeader,
    card_data_responses: HashMap<CardFileID, CardItem<CardResponseParameterData>>,
}

impl CardData {
    pub fn from_data<R: ReadBytes + BinSeek>(
        header: TachographHeader,
        reader: &mut R,
    ) -> Result<CardData> {
        let card_data_responses = <dyn tacho::Card<CardResponseParameterData>>::from_data(
            reader,
            &|card_file_id: CardFileID, data_file: &CardDataFile| {
                CardData::parse_card(card_file_id, data_file)
            },
        )?;

        Ok(Self {
            header,
            card_data_responses,
        })
    }

    fn parse_card(
        card_file_id: CardFileID,
        data_file: &CardDataFile,
    ) -> Result<CardResponseParameterData> {
        debug!("CardData::parse_card - {:?}", card_file_id);
        Ok(CardResponseParameterData::DriverCard)
    }
}
