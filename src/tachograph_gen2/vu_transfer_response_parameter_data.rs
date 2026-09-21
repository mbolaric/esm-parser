use serde::Serialize;

use crate::gen2::{DataInfo, SignatureRecordArray, VUActivity, VUCardDownload, VUEvents, VUOverview, VUSpeed, VUTechnicalData};
use crate::tacho::{VUFileData, VUTransferResponseParameter, VUTransferResponseParameterID};

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VUTransferResponseParameterData"))]
pub enum VUTransferResponseParameterData {
    Unknown(DataInfo),
    Control(VUOverview),
    Activity(VUActivity),
    Events(VUEvents),
    Speed(VUSpeed),
    Calibration(VUTechnicalData),
    CardDownload(VUCardDownload),
    OddballCrashDump,
}

impl VUTransferResponseParameterData {
    pub fn get_signature_record_array(&self) -> Option<&SignatureRecordArray> {
        match self {
            VUTransferResponseParameterData::Control(vu_overview) => Some(&vu_overview.signature_record_array),
            VUTransferResponseParameterData::Activity(vu_activity) => vu_activity.signature_record_array.as_ref(),
            VUTransferResponseParameterData::Events(vu_events) => vu_events.signature_record_array.as_ref(),
            VUTransferResponseParameterData::Speed(vu_speed) => vu_speed.signature_record_array.as_ref(),
            VUTransferResponseParameterData::Calibration(vu_technical_data) => vu_technical_data.signature_record_array.as_ref(),
            VUTransferResponseParameterData::CardDownload(vu_card_download) => vu_card_download.signature_record_array.as_ref(),
            VUTransferResponseParameterData::OddballCrashDump | VUTransferResponseParameterData::Unknown(_) => None,
        }
    }
}

fn extract_from_signature_record_array(
    sig_array: &SignatureRecordArray,
    trep_id: VUTransferResponseParameterID,
    position: u32,
    raw_bytes: &[u8],
    skip_prefix_bytes: usize,
) -> Option<VUFileData> {
    let sig_record_len = 5 + (sig_array.record_size as usize * sig_array.no_of_records as usize);
    if raw_bytes.len() < skip_prefix_bytes + sig_record_len {
        return None;
    }
    let signed_data = raw_bytes[skip_prefix_bytes..raw_bytes.len() - sig_record_len].to_vec();
    let signatures = sig_array.records.clone();
    let signature = signatures.first().cloned();
    Some(VUFileData {
        trep_id,
        position,
        size: signed_data.len() as u32,
        signature,
        signatures,
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
                // Per Regulation (EU) 2016/799 Annex IC Appendix 7 (DDP_029):
                // In TREP 01 / 21 / 31 (Overview), the signature covers all preceding data EXCEPT the certificates.
                let skip_prefix_bytes = (5 + vu_overview.member_state_certificate_record_array.record_size as usize
                    * vu_overview.member_state_certificate_record_array.no_of_records as usize)
                    + (5 + vu_overview.vu_certificate_record_array.record_size as usize
                        * vu_overview.vu_certificate_record_array.no_of_records as usize);
                extract_from_signature_record_array(
                    &vu_overview.signature_record_array,
                    trep_id,
                    position,
                    raw_bytes,
                    skip_prefix_bytes,
                )
            }
            VUTransferResponseParameterData::Activity(vu_activity) => vu_activity
                .signature_record_array
                .as_ref()
                .and_then(|sig| extract_from_signature_record_array(sig, trep_id, position, raw_bytes, 0)),
            VUTransferResponseParameterData::Events(vu_events) => vu_events
                .signature_record_array
                .as_ref()
                .and_then(|sig| extract_from_signature_record_array(sig, trep_id, position, raw_bytes, 0)),
            VUTransferResponseParameterData::Speed(vu_speed) => vu_speed
                .signature_record_array
                .as_ref()
                .and_then(|sig| extract_from_signature_record_array(sig, trep_id, position, raw_bytes, 0)),
            VUTransferResponseParameterData::Calibration(vu_technical_data) => vu_technical_data
                .signature_record_array
                .as_ref()
                .and_then(|sig| extract_from_signature_record_array(sig, trep_id, position, raw_bytes, 0)),
            VUTransferResponseParameterData::CardDownload(vu_card_download) => vu_card_download
                .signature_record_array
                .as_ref()
                .and_then(|sig| extract_from_signature_record_array(sig, trep_id, position, raw_bytes, 0)),
            VUTransferResponseParameterData::OddballCrashDump | VUTransferResponseParameterData::Unknown(_) => None,
        }
    }
}
