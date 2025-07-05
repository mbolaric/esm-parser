use crate::impl_enum_from_u8;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum EventFaultRecordPurpose {
    OneOf10MostRecentOrLast = 0,
    LongestEventForOneOfLast10Days = 1,
    OneOf5LongestEventsOverLast365Days = 2,
    LastEventForOneOfLast10Days = 3,
    MostSeriousEventForOneOfLast10Days = 4,
    OneOf5MostSeriousEventsOverLast365Days = 5,
    FirstEventorFaultAfterLastCalibration = 6,
    ActiveEventOrFault = 7,
    Unknown = 255,
}

impl_enum_from_u8!(
    EventFaultRecordPurpose {
        OneOf10MostRecentOrLast = 0,
        LongestEventForOneOfLast10Days = 1,
        OneOf5LongestEventsOverLast365Days = 2,
        LastEventForOneOfLast10Days = 3,
        MostSeriousEventForOneOfLast10Days = 4,
        OneOf5MostSeriousEventsOverLast365Days = 5,
        FirstEventorFaultAfterLastCalibration = 6,
        ActiveEventOrFault = 7,
        Unknown = 255,
    }
);
