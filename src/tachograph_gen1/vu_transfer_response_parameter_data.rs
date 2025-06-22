use crate::{
    gen1::{VUActivity, VUCalibration, VUControl},
    tacho::VUTransferResponseParameter,
};

#[derive(Debug)]
pub enum VUTransferResponseParameterData {
    Unknown,
    Control(VUControl),
    Activity(VUActivity),
    Events,
    Speed,
    Calibration(VUCalibration),
    CardDownload,
    OddballCrashDump,
}

impl VUTransferResponseParameter for VUTransferResponseParameterData {
    fn is_oddball_crash_dump(&self) -> bool {
        matches!(self, VUTransferResponseParameterData::OddballCrashDump)
    }
}
