use serde::Serialize;

use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum OperationType {
    Reserved = 0,
    Load = 1,
    Unload = 2,
    SimultaneousLoadAndUnload = 3,
    Unknown = 255,
}

impl_enum_from_u8!(
    OperationType {
        Reserved = 0,
        Load = 1,
        Unload = 2,
        SimultaneousLoadAndUnload = 3,
        Unknown = 255,
    }
);
