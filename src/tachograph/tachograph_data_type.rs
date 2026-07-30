use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum TachographDataType {
    VU,
    Card,
}
