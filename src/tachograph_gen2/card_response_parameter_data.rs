use crate::{gen1, gen2};

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Option<Box<gen1::DriverCard>>, Option<Box<gen2::DriverCard>>),
    CompanyCard,
    WorkshopCard(Option<Box<gen1::WorkshopCard>>, Option<Box<gen2::WorkshopCard>>),
    ControlCard,
}
