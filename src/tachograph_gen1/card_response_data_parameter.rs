use crate::gen1::{CompanyCard, ControlCard, DriverCard, WorkshopCard};

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Box<DriverCard>),
    CompanyCard(Box<CompanyCard>),
    WorkshopCard(Box<WorkshopCard>),
    ControlCard(Box<ControlCard>),
}
