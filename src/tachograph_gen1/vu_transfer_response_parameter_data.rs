use serde::Serialize;

use crate::{
    gen1::{VUActivity, VuDetailedSpeed, VuEvents, VuOverview, VuTechnicalData},
    tacho::VUTransferResponseParameter,
};

#[derive(Debug, Serialize)]
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

impl VUTransferResponseParameter for VUTransferResponseParameterData {
    fn is_oddball_crash_dump(&self) -> bool {
        matches!(self, VUTransferResponseParameterData::OddballCrashDump)
    }
}
