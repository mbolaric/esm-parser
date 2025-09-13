use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum TachographDataGeneration {
    Unknown,
    FirstGeneration,
    SecondGeneration,
}
