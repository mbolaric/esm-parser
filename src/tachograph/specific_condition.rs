use crate::impl_enum_from_u8;

#[derive(Debug)]
#[repr(u8)]
pub enum SpecificCondition {
    OutOfScopeBegin = 1,
    OutOfScopeEnd = 2,
    FerryTrainCrossing = 3,
    FerryTrainCrossingEnd = 4,
    Unknown = 0xFF,
}

impl_enum_from_u8!(
    SpecificCondition {
        OutOfScopeBegin = 1,
        OutOfScopeEnd = 2,
        FerryTrainCrossing = 3,
        FerryTrainCrossingEnd = 4,
        Unknown = 0xFF,
    }
);
