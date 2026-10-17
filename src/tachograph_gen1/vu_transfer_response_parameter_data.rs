use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

use crate::gen1::{VUActivity, VuDetailedSpeed, VuEvents, VuOverview, VuTechnicalData};
use crate::tacho::{VUFileData, VUTransferResponseParameter, VUTransferResponseParameterID};

#[derive(Debug)]
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

impl Serialize for VUTransferResponseParameterData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fn serialize_variant<S: Serializer, T: Serialize>(
            serializer: S,
            name: &'static str,
            value: &T,
        ) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("VUTransferResponseParameterData", 1)?;
            state.serialize_field(name, value)?;
            state.end()
        }

        match self {
            Self::Unknown => serializer.serialize_str("Unknown"),
            Self::Control(v) => serialize_variant(serializer, "Control", v),
            Self::Activity(v) => serialize_variant(serializer, "Activity", v),
            Self::Events(v) => serialize_variant(serializer, "Events", v),
            Self::Speed(v) => serialize_variant(serializer, "Speed", v),
            Self::Calibration(v) => serialize_variant(serializer, "Calibration", v),
            Self::CardDownload => serializer.serialize_str("CardDownload"),
            Self::OddballCrashDump => serializer.serialize_str("OddballCrashDump"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Export;
    use crate::gen1::VuDetailedSpeedData;
    use crate::tacho::VUTransferResponseParameterItem;

    #[test]
    fn test_gen1_vu_transfer_response_parameter_data_json_and_xml() {
        let item = VUTransferResponseParameterItem {
            type_id: VUTransferResponseParameterID::Speed,
            position: 0,
            data: VUTransferResponseParameterData::Speed(VuDetailedSpeed {
                vu_detailed_speed_data: VuDetailedSpeedData { no_of_speed_blocks: 0, vu_detailed_speed_blocks: vec![] },
                signature: None,
            }),
        };

        let json = item.to_json().unwrap();
        assert!(json.contains("\"Speed\":"));
        let xml = item.to_xml().unwrap();
        assert!(xml.contains("<Speed>"));
    }
}
