use serde::{
    Serialize,
    ser::{SerializeStruct, Serializer},
};

use crate::{gen1, gen2, tacho::CardParser};

#[derive(Debug)]
pub enum ParsedCard<TGen1: CardParser<TGen1>, TGen2: CardParser<TGen2>> {
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

#[derive(Debug)]
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
