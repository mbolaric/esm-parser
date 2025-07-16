use crate::{
    gen2::{DataInfo, VUActivity, VUCardDownload, VUEvents, VUOverview, VUSpeed, VUTechnicalData},
    tacho::VUTransferResponseParameter,
};

#[derive(Debug)]
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
