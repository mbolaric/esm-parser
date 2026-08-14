use serde::Serialize;

use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[repr(u8)]
pub enum LoadType {
    Undefined = 0,
    Goods = 1,
    Passengers = 2,
    Unknown = 255,
}

impl_enum_from_u8!(
    LoadType {
        Undefined = 0,
        Goods = 1,
        Passengers = 2,
        Unknown = 255,
    }
);
