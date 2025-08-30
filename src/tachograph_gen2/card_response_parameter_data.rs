use crate::{gen1, gen2, tacho::CardParser};

#[derive(Debug)]
pub enum ParsedCard<TGen1: CardParser<TGen1>, TGen2: CardParser<TGen2>> {
    Gen1(Box<TGen1>),
    Gen2(Box<TGen2>),
    Combined(Box<TGen1>, Box<TGen2>),
    None,
}

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(ParsedCard<gen1::DriverCard, gen2::DriverCard>),
    CompanyCard,
    WorkshopCard(ParsedCard<gen1::WorkshopCard, gen2::WorkshopCard>),
    ControlCard,
}
