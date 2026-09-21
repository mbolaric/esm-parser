use serde::Serialize;

use crate::Export;
use crate::tacho::{CardFileID, TimeReal, VUTransferResponseParameterID};

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyStatus {
    Invalid,
    Valid,
    InvalidSignatureSize,
    NotHaveSignature,
    NotHaveData,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyItem {
    pub card_file_id: CardFileID,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

/// Verification result item for a downloaded Vehicle Unit (VU) data record (TREP).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VUVerifyItem {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    pub position: u32,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyResultStatus {
    Invalid,
    Valid,
    Unsigned,
    PartiallyValid,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VerifyItem>,
}

impl Export for VerifyResult {}

/// Verification result for all downloaded Vehicle Unit (VU) data records and signatures.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VUVerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VUVerifyItem>,
}

impl Export for VUVerifyResult {}

/// Identifies which certificate a `VuVerifyItem` reports on.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VuCertificateKind {
    MemberStateCertificate,
    VuCertificate,
}

/// Verification item for a certificate within a VU Overview certificate chain (`ERCA -> MSCA -> VU`).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuVerifyItem {
    pub certificate: VuCertificateKind,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

/// Descriptive alias for `VuVerifyItem` to distinguish overview certificate checks from full data checks.
pub type VuOverviewCertificateVerifyItem = VuVerifyItem;

/// Verification result for a Vehicle Unit (VU) Overview certificate chain (`ERCA -> MSCA -> VU`).
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuVerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VuVerifyItem>,
}

impl Export for VuVerifyResult {}

/// Descriptive alias for `VuVerifyResult` to distinguish overview certificate checks from full data checks.
pub type VuOverviewCertificateVerifyResult = VuVerifyResult;
