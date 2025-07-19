use crate::gen2::DriverCard;

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Box<DriverCard>),
    CompanyCard,
    WorkshopCard,
    ControlCard,
}
