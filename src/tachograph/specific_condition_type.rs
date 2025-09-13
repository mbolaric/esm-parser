use crate::impl_enum_from_u8;

/// Code identifying a specific condition (Annex 1B requirements 050b,
/// 105a, 212a and 230a and Annex 1C requirements 62).
#[derive(Debug)]
#[repr(u8)]
pub enum SpecificConditionType {
    OutOfScopeBegin = 1,
    OutOfScopeEnd = 2,
    FerryTrainCrossing = 3,
    FerryTrainCrossingEnd = 4,
    Unknown = 0xFF,
}

impl_enum_from_u8!(
    SpecificConditionType {
        OutOfScopeBegin = 1,
        OutOfScopeEnd = 2,
        FerryTrainCrossing = 3,
        FerryTrainCrossingEnd = 4,
        Unknown = 0xFF,
    }
);
