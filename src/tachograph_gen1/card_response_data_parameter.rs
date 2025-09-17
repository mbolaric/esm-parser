use serde::Serialize;

use crate::gen1::{CompanyCard, ControlCard, DriverCard, WorkshopCard};

#[derive(Debug, Serialize)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Box<DriverCard>),
    CompanyCard(Box<CompanyCard>),
    WorkshopCard(Box<WorkshopCard>),
    ControlCard(Box<ControlCard>),
}
