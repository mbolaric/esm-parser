#[derive(Debug, Clone, PartialEq)]
pub enum VUTransferResponseParameterID {
    Unknown = 0x00,
    Control = 0x01,
    Activity = 0x02,
    Events = 0x03,
    Speed = 0x04,
    Calibration = 0x05,
    CardDownload = 0x06,
    OddballCrashDump = 0x11,
    Gen2Control = 0x21,
    Gen2Activity = 0x22,
    Gen2Events = 0x23,
    Gen2Speed = 0x24,
    Gen2Calibration = 0x25,
    Gen2CardDownload = 0x26,
    Gen2v2Control = 0x31,
    Gen2v2Activity = 0x32,
    Gen2v2Events = 0x33,
    Gen2v2Speed = 0x34,
    Gen2v2Calibration = 0x35,
}

impl VUTransferResponseParameterID {
    pub fn is_unknown(&self) -> bool {
        matches!(*self, VUTransferResponseParameterID::Unknown)
    }
}

impl From<u8> for VUTransferResponseParameterID {
    fn from(value: u8) -> Self {
        match value {
            0x01 => VUTransferResponseParameterID::Control,
            0x02 => VUTransferResponseParameterID::Activity,
            0x03 => VUTransferResponseParameterID::Events,
            0x04 => VUTransferResponseParameterID::Speed,
            0x05 => VUTransferResponseParameterID::Calibration,
            0x06 => VUTransferResponseParameterID::CardDownload,
            0x11 => VUTransferResponseParameterID::OddballCrashDump,
            0x21 => VUTransferResponseParameterID::Gen2Control,
            0x22 => VUTransferResponseParameterID::Gen2Activity,
            0x23 => VUTransferResponseParameterID::Gen2Events,
            0x24 => VUTransferResponseParameterID::Gen2Speed,
            0x25 => VUTransferResponseParameterID::Gen2Calibration,
            0x26 => VUTransferResponseParameterID::Gen2CardDownload,
            0x31 => VUTransferResponseParameterID::Gen2v2Control,
            0x32 => VUTransferResponseParameterID::Gen2v2Activity,
            0x33 => VUTransferResponseParameterID::Gen2v2Events,
            0x34 => VUTransferResponseParameterID::Gen2v2Speed,
            0x35 => VUTransferResponseParameterID::Gen2v2Calibration,
            _ => VUTransferResponseParameterID::Unknown,
        }
    }
}
