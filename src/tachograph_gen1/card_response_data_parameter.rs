use crate::gen1::Card;

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Card),
    CompanyCard(Card),
    WorkshopCard(Card),
    ControlCard(Card),
}
