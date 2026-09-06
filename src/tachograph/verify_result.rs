use serde::Serialize;

use crate::Export;
use crate::tacho::{CardFileID, TimeReal};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyStatus {
    Invalid,
    Valid,
    InvalidSignatureSize,
    NotHaveSignature,
    NotHaveData,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyItem {
    pub card_file_id: CardFileID,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyResultStatus {
    Invalid,
    Valid,
    Unsigned,
    PartiallyValid,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VerifyItem>,
}

impl Export for VerifyResult {}

/// Identifies which certificate a `VuVerifyItem` reports on.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VuCertificateKind {
    MemberStateCertificate,
    VuCertificate,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuVerifyItem {
    pub certificate: VuCertificateKind,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuVerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VuVerifyItem>,
}

impl Export for VuVerifyResult {}
