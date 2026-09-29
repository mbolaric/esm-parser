use serde::Serialize;

use crate::gen1::PlaceRecord;
use crate::tacho::FullCardNumber;
use crate::{Readable, read_records};

/// Information, stored in a vehicle unit, related to a place where a driver
/// begins or ends a daily work period (Annex 1B requirement 087 and
/// Annex 1C requirement 108 and 110).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuPlaceDailyWorkPeriodRecord"))]
pub struct VuPlaceDailyWorkPeriodRecord {
    #[serde(rename = "fullCardNumber")]
    pub full_card_number: FullCardNumber,
    #[serde(rename = "placeRecord")]
    pub place_record: PlaceRecord,
}

impl Readable for VuPlaceDailyWorkPeriodRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuPlaceDailyWorkPeriodRecord> {
        let full_card_number = FullCardNumber::read(reader)?;
        let place_record = PlaceRecord::read(reader)?;
        Ok(Self { full_card_number, place_record })
    }
}

/// Information, stored in a vehicle unit, related to places where drivers
/// begin or end a daily work period (Annex 1B requirement 087 and
/// Annex 1C requirement 108 and 110).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen1VuPlaceDailyWorkPeriodData"))]
pub struct VuPlaceDailyWorkPeriodData {
    #[serde(rename = "noOfPlaceRecords")]
    pub no_of_place_records: u8,
    #[serde(rename = "vuPlaceDailyWorkPeriodRecords")]
    pub vu_place_daily_work_period_records: Vec<VuPlaceDailyWorkPeriodRecord>,
}

impl Readable for VuPlaceDailyWorkPeriodData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuPlaceDailyWorkPeriodData> {
        let no_of_place_records = reader.read_u8()?;
        let vu_place_daily_work_period_records =
            read_records(reader, no_of_place_records as usize, |reader| VuPlaceDailyWorkPeriodRecord::read(reader))?;
        Ok(Self { no_of_place_records, vu_place_daily_work_period_records })
    }
}
