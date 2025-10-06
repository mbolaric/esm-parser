use serde::Serialize;

use crate::tacho::{CardFileID, TimeReal};

#[derive(Debug, Serialize)]
pub enum VerifyStatus {
    Invalid,
    Valid,
    InvalidSignatureSize,
    NotHaveSignature,
    NotHaveData,
}

#[derive(Debug, Serialize)]
pub struct VerifyItem {
    pub card_file_id: CardFileID,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

#[derive(Debug, Serialize)]
pub enum VerifyResultStatus {
    Invalid,
    Valid,
    Unsigned,
    PartialyValid,
}

#[derive(Debug, Serialize)]
pub struct VerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VerifyItem>,
}
