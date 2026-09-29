use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum TachographDataGeneration {
    Unknown,
    FirstGeneration,
    SecondGeneration,
}
