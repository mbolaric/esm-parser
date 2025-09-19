use serde::Serialize;

use crate::impl_enum_from_u8;

/// Code indicating an activity carried out by a company using its company card.
#[derive(Debug, PartialEq, Serialize)]
#[repr(u8)]
pub enum CompanyActivityType {
    Unknown = 0,
    CardDownloading = 1,
    VUDownloading = 2,
    VULockin = 3,
    VULockout = 4,
}

impl_enum_from_u8!(
    CompanyActivityType {
        Unknown = 0,
        CardDownloading = 1,
        VUDownloading = 2,
        VULockin = 3,
        VULockout = 4,
    }
);
