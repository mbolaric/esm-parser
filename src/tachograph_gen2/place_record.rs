use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::GnssPlaceRecord,
    tacho::{EntryTypeDailyWorkPeriod, NationNumeric, OdometerShort, RegionNumeric, TimeReal},
};

#[derive(Debug)]
pub struct PlaceRecord {
    pub entry_time: TimeReal,
    pub entry_type_daily_work_period: EntryTypeDailyWorkPeriod,
    pub daily_work_period_country: NationNumeric,
    pub daily_work_period_region: RegionNumeric,
    pub vehicle_odometer_value: OdometerShort,
    pub gnns_place_record: GnssPlaceRecord,
}

impl Readable<PlaceRecord> for PlaceRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<PlaceRecord> {
        let entry_time = TimeReal::read(reader)?;
        let entry_type_daily_work_period: EntryTypeDailyWorkPeriod = reader.read_u8()?.into();
        let daily_work_period_country: NationNumeric = reader.read_u8()?.into();
        let daily_work_period_region: RegionNumeric = reader.read_u8()?.into();
        let vehicle_odometer_value = OdometerShort::read(reader)?;
        let gnns_place_record = GnssPlaceRecord::read(reader)?;

        Ok(Self {
            entry_time,
            entry_type_daily_work_period,
            daily_work_period_country,
            daily_work_period_region,
            vehicle_odometer_value,
            gnns_place_record,
        })
    }
}
