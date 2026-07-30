use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

use crate::tacho::CardParser;
use crate::{gen1, gen2};

#[cfg(feature = "typescript")]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen2NoParsedCard")]
#[ts(type = "null")]
struct NoParsedCard;

#[cfg(feature = "typescript")]
#[allow(dead_code)]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen2ParsedCardOutput")]
#[ts(untagged)]
enum ParsedCardOutput<TGen1, TGen2> {
    Gen1 { gen1: Box<TGen1> },
    Gen2 { gen2: Box<TGen2> },
    Combined { gen1: Box<TGen1>, gen2: Box<TGen2> },
    None(NoParsedCard),
}

#[derive(Debug)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2ParsedCard"))]
#[cfg_attr(feature = "typescript", ts(as = "ParsedCardOutput<TGen1, TGen2>"))]
pub enum ParsedCard<TGen1, TGen2> {
    Gen1(Box<TGen1>),
    Gen2(Box<TGen2>),
    Combined(Box<TGen1>, Box<TGen2>),
    None,
}

impl<TGen1, TGen2> Serialize for ParsedCard<TGen1, TGen2>
where
    TGen1: CardParser<TGen1> + Serialize,
    TGen2: CardParser<TGen2> + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParsedCard::Gen1(data) => {
                let mut state = serializer.serialize_struct("Gen1", 1)?;
                state.serialize_field("gen1", data)?;
                state.end()
            }
            ParsedCard::Gen2(data) => {
                let mut state = serializer.serialize_struct("Gen2", 1)?;
                state.serialize_field("gen2", data)?;
                state.end()
            }
            ParsedCard::Combined(gen1_data, gen2_data) => {
                let mut state = serializer.serialize_struct("Combined", 2)?;
                state.serialize_field("gen1", gen1_data)?;
                state.serialize_field("gen2", gen2_data)?;
                state.end()
            }
            ParsedCard::None => serializer.serialize_none(),
        }
    }
}

#[cfg(feature = "typescript")]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen2UnsupportedResponse")]
#[ts(type = "\"Unsupported\"")]
struct UnsupportedResponse;

#[cfg(feature = "typescript")]
#[allow(dead_code)]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen2CardResponseParameterDataOutput")]
#[ts(untagged)]
enum CardResponseParameterDataOutput {
    Unsupported(UnsupportedResponse),
    DriverCard(ParsedCard<gen1::DriverCard, gen2::DriverCard>),
    CompanyCard(ParsedCard<gen1::CompanyCard, gen2::CompanyCard>),
    WorkshopCard(ParsedCard<gen1::WorkshopCard, gen2::WorkshopCard>),
    ControlCard(ParsedCard<gen1::ControlCard, gen2::ControlCard>),
}

#[derive(Debug)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2CardResponseParameterData"))]
#[cfg_attr(feature = "typescript", ts(as = "CardResponseParameterDataOutput"))]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(ParsedCard<gen1::DriverCard, gen2::DriverCard>),
    CompanyCard(ParsedCard<gen1::CompanyCard, gen2::CompanyCard>),
    WorkshopCard(ParsedCard<gen1::WorkshopCard, gen2::WorkshopCard>),
    ControlCard(ParsedCard<gen1::ControlCard, gen2::ControlCard>),
}

impl Serialize for CardResponseParameterData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CardResponseParameterData::DriverCard(data) => data.serialize(serializer),
            CardResponseParameterData::CompanyCard(data) => data.serialize(serializer),
            CardResponseParameterData::WorkshopCard(data) => data.serialize(serializer),
            CardResponseParameterData::ControlCard(data) => data.serialize(serializer),
            CardResponseParameterData::Unsupported => serializer.serialize_str("Unsupported"),
        }
    }
}
