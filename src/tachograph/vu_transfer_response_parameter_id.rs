use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum VUTransferResponseParameterID {
    Unknown = 0x00,
    Overview = 0x01,
    Activities = 0x02,
    EventsAndFaults = 0x03,
    Speed = 0x04,
    TechnicalData = 0x05,
    CardDownload = 0x06,
    OddballCrashDump = 0x11,
    Gen2Overview = 0x21,
    Gen2Activities = 0x22,
    Gen2EventsAndFaults = 0x23,
    Gen2Speed = 0x24,
    Gen2TechnicalData = 0x25,
    Gen2CardDownload = 0x26,
    Gen2v2Overview = 0x31,
    Gen2v2Activities = 0x32,
    Gen2v2EventsAndFaults = 0x33,
    Gen2v2Speed = 0x34,
    Gen2v2TechnicalData = 0x35,
}

impl VUTransferResponseParameterID {
    pub fn is_unknown(&self) -> bool {
        matches!(*self, VUTransferResponseParameterID::Unknown)
    }
}

impl From<u8> for VUTransferResponseParameterID {
    fn from(value: u8) -> Self {
        match value {
            0x01 => VUTransferResponseParameterID::Overview,
            0x02 => VUTransferResponseParameterID::Activities,
            0x03 => VUTransferResponseParameterID::EventsAndFaults,
            0x04 => VUTransferResponseParameterID::Speed,
            0x05 => VUTransferResponseParameterID::TechnicalData,
            0x06 => VUTransferResponseParameterID::CardDownload,
            0x11 => VUTransferResponseParameterID::OddballCrashDump,
            0x21 => VUTransferResponseParameterID::Gen2Overview,
            0x22 => VUTransferResponseParameterID::Gen2Activities,
            0x23 => VUTransferResponseParameterID::Gen2EventsAndFaults,
            0x24 => VUTransferResponseParameterID::Gen2Speed,
            0x25 => VUTransferResponseParameterID::Gen2TechnicalData,
            0x26 => VUTransferResponseParameterID::Gen2CardDownload,
            0x31 => VUTransferResponseParameterID::Gen2v2Overview,
            0x32 => VUTransferResponseParameterID::Gen2v2Activities,
            0x33 => VUTransferResponseParameterID::Gen2v2EventsAndFaults,
            0x34 => VUTransferResponseParameterID::Gen2v2Speed,
            0x35 => VUTransferResponseParameterID::Gen2v2TechnicalData,
            _ => VUTransferResponseParameterID::Unknown,
        }
    }
}
