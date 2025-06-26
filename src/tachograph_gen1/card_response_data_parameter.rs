use crate::gen1::Card;

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Card),
    CompanyCard,
    WorkshopCard(Card),
    ControlCard,
    Unknown(Card),
}
