use serde::{Serialize, ser::Serializer};

use crate::gen1::{CompanyCard, ControlCard, DriverCard, WorkshopCard};

#[derive(Debug)]
pub enum CardResponseParameterData {
    Unsupported,
    DriverCard(Box<DriverCard>),
    CompanyCard(Box<CompanyCard>),
    WorkshopCard(Box<WorkshopCard>),
    ControlCard(Box<ControlCard>),
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
