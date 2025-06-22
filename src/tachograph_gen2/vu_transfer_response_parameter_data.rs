use crate::{
    gen2::{DataInfo, VUActivity, VUCalibration, VUCardDownload, VUControl, VUEvents, VUSpeed},
    tacho::VUTransferResponseParameter,
};

#[derive(Debug)]
pub enum VUTransferResponseParameterData {
    Unknown(DataInfo),
    Control(VUControl),
    Activity(VUActivity),
    Events(VUEvents),
    Speed(VUSpeed),
    Calibration(VUCalibration),
    CardDownload(VUCardDownload),
    OddballCrashDump,
}

impl VUTransferResponseParameter for VUTransferResponseParameterData {
    fn is_oddball_crash_dump(&self) -> bool {
        matches!(self, VUTransferResponseParameterData::OddballCrashDump)
    }
}
