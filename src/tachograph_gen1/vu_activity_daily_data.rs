use binary_data::BigEndian;
use serde::Serialize;

use crate::tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams};
use crate::{Readable, ReadableWithParams, read_records};

/// Information, stored in a VU, related to changes of activity and/or
/// changes of driving status and/or changes of card status for a given
/// calendar day (Annex 1B requirement 084 and Annex 1C requirement
/// 105, 106, 107) and to slots status at 00:00 that day.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuActivityDailyData"))]
pub struct VuActivityDailyData {
    #[serde(rename = "noOfActivityChanges")]
    pub no_of_activity_changes: u16,
    #[serde(rename = "activityChangeInfos")]
    pub activity_change_infos: Vec<ActivityChangeInfo>,
}

impl Readable for VuActivityDailyData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuActivityDailyData> {
        let no_of_activity_changes = reader.read_u16::<BigEndian>()?;
        let params = ActivityChangeInfoParams::new(ActivityCard::Vu);
        let activity_change_infos =
            read_records(reader, no_of_activity_changes as usize, |reader| ActivityChangeInfo::read(reader, &params))?;
        Ok(Self { no_of_activity_changes, activity_change_infos })
    }
}
