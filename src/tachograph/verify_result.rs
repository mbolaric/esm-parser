use serde::Serialize;

use crate::Export;
use crate::tacho::{CardFileID, TimeReal, VUTransferResponseParameterID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyStatus {
    Invalid,
    Valid,
    InvalidSignatureSize,
    NotHaveSignature,
    NotHaveData,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyItem {
    pub card_file_id: CardFileID,
    pub status: VerifyStatus,
    pub end_of_validity: Option<TimeReal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VerifyResultStatus {
    Invalid,
    Valid,
    Unsigned,
    PartiallyValid,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VerifyItem>,
}

impl Export for VerifyResult {}

/// Identifies which certificate a `VuVerifyItem::Certificate` reports on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum VuCertificateKind {
    MemberStateCertificate,
    VuCertificate,
}

/// Verification item for a Vehicle Unit (VU) check (certificate chain item or downloaded data record).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(untagged)]
pub enum VuVerifyItem {
    Certificate {
        certificate: VuCertificateKind,
        status: VerifyStatus,
        end_of_validity: Option<TimeReal>,
    },
    Record {
        #[serde(rename = "trepId")]
        trep_id: VUTransferResponseParameterID,
        position: u32,
        status: VerifyStatus,
        end_of_validity: Option<TimeReal>,
    },
}

impl VuVerifyItem {
    pub fn certificate(certificate: VuCertificateKind, status: VerifyStatus, end_of_validity: Option<TimeReal>) -> Self {
        Self::Certificate { certificate, status, end_of_validity }
    }

    pub fn record(
        trep_id: VUTransferResponseParameterID,
        position: u32,
        status: VerifyStatus,
        end_of_validity: Option<TimeReal>,
    ) -> Self {
        Self::Record { trep_id, position, status, end_of_validity }
    }

    pub fn status(&self) -> &VerifyStatus {
        match self {
            Self::Certificate { status, .. } | Self::Record { status, .. } => status,
        }
    }

    pub fn end_of_validity(&self) -> Option<&TimeReal> {
        match self {
            Self::Certificate { end_of_validity, .. } | Self::Record { end_of_validity, .. } => end_of_validity.as_ref(),
        }
    }

    pub fn trep_id(&self) -> Option<&VUTransferResponseParameterID> {
        match self {
            Self::Record { trep_id, .. } => Some(trep_id),
            _ => None,
        }
    }

    pub fn certificate_kind(&self) -> Option<&VuCertificateKind> {
        match self {
            Self::Certificate { certificate, .. } => Some(certificate),
            _ => None,
        }
    }

    pub fn position(&self) -> Option<u32> {
        match self {
            Self::Record { position, .. } => Some(*position),
            _ => None,
        }
    }
}

/// Verification result for Vehicle Unit (VU) checks (certificate chain or downloaded data records).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct VuVerifyResult {
    pub status: VerifyResultStatus,
    pub result: Vec<VuVerifyItem>,
}

impl Export for VuVerifyResult {}
