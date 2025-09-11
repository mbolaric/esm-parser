use crate::impl_enum_from_u8;

/// Code indicating the activities carried out during a control. This data type
/// is related to Annex 1C requirements 126, 274, 299, 327, and 350.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ControlType {
    /// Card downloaded/not downloaded during this control activity,
    CardDownloaded = 0x80,
    /// VU downloaded/not downloaded during this control activity,
    VUDownloaded = 0x40,
    /// Printing done/no printing done during this control activity,
    /// 0x20 (32)
    PrintingDone = 0x20,
    /// Display used/no display used during this control activity,
    DisplayUsed = 0x10,
    /// Calibration parameters checked/not checked during this control activity,
    CalibrationParameters = 0x8,
    /// Control Type are invalid
    Unknown = 0x0,
}

impl_enum_from_u8!(
    ControlType {
        CardDownloaded = 0x80,
        VUDownloaded = 0x40,
        PrintingDone = 0x20,
        DisplayUsed = 0x10,
        CalibrationParameters = 0x8,
        Unknown = 0x0
    }
);
