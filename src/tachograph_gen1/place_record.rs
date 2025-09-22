use serde::Serialize;

use crate::{
    Readable,
    tacho::{CardPlace, EntryTypeDailyWorkPeriod, NationNumeric, OdometerShort, RegionNumeric, TimeReal},
};

/// Information related to a place where a daily work period begins or ends
/// (Annex 1C requirements 108, 271, 296, 324, and 347).
#[derive(Debug, Serialize)]
pub struct PlaceRecord {
    #[serde(rename = "entryTime")]
    pub entry_time: TimeReal,
    #[serde(rename = "entryTypeDailyWorkPeriod")]
    pub entry_type_daily_work_period: EntryTypeDailyWorkPeriod,
    #[serde(rename = "dailyWorkPeriodCountry")]
    pub daily_work_period_country: NationNumeric,
    #[serde(rename = "dailyWorkPeriodRegion")]
    pub daily_work_period_region: RegionNumeric,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<PlaceRecord> for PlaceRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<PlaceRecord> {
        let entry_time = TimeReal::read(reader)?;
        let entry_type_daily_work_period: EntryTypeDailyWorkPeriod = reader.read_u8()?.into();
        let daily_work_period_country: NationNumeric = reader.read_u8()?.into();
        let daily_work_period_region: RegionNumeric = reader.read_u8()?.into();
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self {
            entry_time,
            entry_type_daily_work_period,
            daily_work_period_country,
            daily_work_period_region,
            vehicle_odometer_value,
        })
    }
}

impl CardPlace for PlaceRecord {
    fn get_entry_time(&self) -> &TimeReal {
        &self.entry_time
    }
}
