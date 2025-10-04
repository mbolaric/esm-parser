use serde::Serialize;

use crate::tacho::CardFileID;

#[derive(Debug, Serialize)]
pub enum VerifyStatus {
    Invalid,
    Valid,
    NotHaveSignature,
}

#[derive(Debug, Serialize)]
pub struct VerifyResult {
    pub card_file_id: CardFileID,
    pub status: VerifyStatus,
}
