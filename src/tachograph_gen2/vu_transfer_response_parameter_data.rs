use serde::Serialize;

use crate::gen2::{DataInfo, VUActivity, VUCardDownload, VUEvents, VUOverview, VUSpeed, VUTechnicalData};
use crate::tacho::VUTransferResponseParameter;

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

impl VUTransferResponseParameter for VUTransferResponseParameterData {
    fn is_oddball_crash_dump(&self) -> bool {
        matches!(self, VUTransferResponseParameterData::OddballCrashDump)
    }
}
