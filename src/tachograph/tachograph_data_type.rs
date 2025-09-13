use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum TachographDataType {
    VU,
    Card,
}
