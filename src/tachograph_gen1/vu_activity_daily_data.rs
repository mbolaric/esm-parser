use binary_data::BigEndian;
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams,
    tacho::{ActivityCard, ActivityChangeInfo, ActivityChangeInfoParams},
};

#[derive(Debug, Serialize)]
pub struct VuActivityDailyData {
    pub no_of_activity_changes: u16,
    pub activity_change_infos: Vec<ActivityChangeInfo>,
}

impl Readable<VuActivityDailyData> for VuActivityDailyData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuActivityDailyData> {
        let no_of_activity_changes = reader.read_u16::<BigEndian>()?;
        let mut activity_change_infos: Vec<ActivityChangeInfo> = Vec::with_capacity(no_of_activity_changes as usize);
        let params = ActivityChangeInfoParams::new(ActivityCard::Vu);
        for _ in 0..no_of_activity_changes {
            let item = ActivityChangeInfo::read(reader, &params)?;
            activity_change_infos.push(item);
        }
        Ok(Self { no_of_activity_changes, activity_change_infos })
    }
}
