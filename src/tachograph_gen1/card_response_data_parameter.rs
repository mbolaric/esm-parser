use serde::Serialize;
use serde::ser::Serializer;

use crate::gen1::{CompanyCard, ControlCard, DriverCard, WorkshopCard};

#[cfg(feature = "typescript")]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen1UnsupportedResponse")]
#[ts(type = "\"Unsupported\"")]
struct UnsupportedResponse;

#[cfg(feature = "typescript")]
#[allow(dead_code)]
#[derive(ts_rs::TS)]
#[ts(rename = "Gen1CardResponseParameterDataOutput")]
#[ts(untagged)]
enum CardResponseParameterDataOutput {
    Unsupported(UnsupportedResponse),
    DriverCard(Box<DriverCard>),
    CompanyCard(Box<CompanyCard>),
    WorkshopCard(Box<WorkshopCard>),
    ControlCard(Box<ControlCard>),
}

#[derive(Debug)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1CardResponseParameterData"))]
#[cfg_attr(feature = "typescript", ts(as = "CardResponseParameterDataOutput"))]
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
