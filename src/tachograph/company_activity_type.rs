use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq)]
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
