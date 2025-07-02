use crate::{
    Readable,
    tacho::{CardPlace, EntryTypeDailyWorkPeriod, NationNumericCode, OdometerShort, RegionNumeric, TimeReal},
};

#[derive(Debug)]
pub struct PlaceRecord {
    pub entry_time: TimeReal,
    pub entry_type_daily_work_period: EntryTypeDailyWorkPeriod,
    pub daily_work_period_country: NationNumericCode,
    pub daily_work_period_region: RegionNumeric,
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<PlaceRecord> for PlaceRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<PlaceRecord> {
        let entry_time = TimeReal::read(reader)?;
        let entry_type_daily_work_period: EntryTypeDailyWorkPeriod = reader.read_u8()?.into();
        let daily_work_period_country: NationNumericCode = reader.read_u8()?.into();
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
