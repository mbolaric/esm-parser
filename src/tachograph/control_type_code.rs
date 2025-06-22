use crate::impl_enum_from_u8;

// FIXME: Check if thet are all posibilities
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ControlTypeCode {
    CardDownloaded = 0x80,
    VUDownloaded = 0x40,
    PrintingDisplay = 0x30,
    PrintingDone = 0x20,
    DisplayUsed = 0x10,
    Unknown = 0x0,
}

impl_enum_from_u8!(
    ControlTypeCode {
        CardDownloaded = 0x80,
        VUDownloaded = 0x40,
        PrintingDisplay = 0x30,
        PrintingDone = 0x20,
        DisplayUsed = 0x10,
        Unknown = 0x0
    }
);
