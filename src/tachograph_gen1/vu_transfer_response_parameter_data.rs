use serde::Serialize;

use crate::gen1::{VUActivity, VuDetailedSpeed, VuEvents, VuOverview, VuTechnicalData};
use crate::tacho::{VUFileData, VUTransferResponseParameter, VUTransferResponseParameterID};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VUTransferResponseParameterData"))]
pub enum VUTransferResponseParameterData {
    Unknown,
    Control(VuOverview),
    Activity(VUActivity),
    Events(VuEvents),
    Speed(VuDetailedSpeed),
    Calibration(VuTechnicalData),
    CardDownload,
    OddballCrashDump,
}

fn extract_from_gen1_signature(
    signature: Option<&Vec<u8>>,
    trep_id: VUTransferResponseParameterID,
    position: u32,
    raw_bytes: &[u8],
    skip_prefix_bytes: usize,
) -> Option<VUFileData> {
    let sig = signature?;
    let sig_len = sig.len();
    if raw_bytes.len() < skip_prefix_bytes + sig_len {
        return None;
    }
    let signed_data = raw_bytes[skip_prefix_bytes..raw_bytes.len() - sig_len].to_vec();
    Some(VUFileData {
        trep_id,
        position,
        size: signed_data.len() as u32,
        signature: Some(sig.clone()),
        signatures: vec![sig.clone()],
        data: Some(signed_data),
        raw_data: Some(raw_bytes.to_vec()),
    })
}

impl VUTransferResponseParameter for VUTransferResponseParameterData {
    fn is_oddball_crash_dump(&self) -> bool {
        matches!(self, VUTransferResponseParameterData::OddballCrashDump)
    }

    fn extract_file_data(&self, trep_id: VUTransferResponseParameterID, position: u32, raw_bytes: &[u8]) -> Option<VUFileData> {
        match self {
            VUTransferResponseParameterData::Control(vu_overview) => {
                // Per Regulation (EC) No 1360/2002 Annex 1B Appendix 7 (DDP_029):
                // In TREP 01 (Overview), signature covers all preceding data except the certificates (194 + 194 = 388 bytes).
                let skip_prefix_bytes = 194 + 194;
                extract_from_gen1_signature(vu_overview.signature.as_ref(), trep_id, position, raw_bytes, skip_prefix_bytes)
            }
            VUTransferResponseParameterData::Activity(vu_activity) => {
                extract_from_gen1_signature(vu_activity.signature.as_ref(), trep_id, position, raw_bytes, 0)
            }
            VUTransferResponseParameterData::Events(vu_events) => {
                extract_from_gen1_signature(vu_events.signature.as_ref(), trep_id, position, raw_bytes, 0)
            }
            VUTransferResponseParameterData::Speed(vu_speed) => {
                extract_from_gen1_signature(vu_speed.signature.as_ref(), trep_id, position, raw_bytes, 0)
            }
            VUTransferResponseParameterData::Calibration(vu_technical_data) => {
                extract_from_gen1_signature(vu_technical_data.signature.as_ref(), trep_id, position, raw_bytes, 0)
            }
            VUTransferResponseParameterData::CardDownload
            | VUTransferResponseParameterData::OddballCrashDump
            | VUTransferResponseParameterData::Unknown => None,
        }
    }
}
