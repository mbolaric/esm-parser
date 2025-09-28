use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen1::{VuActivityDailyData, VuCardIWData, VuPlaceDailyWorkPeriodData, VuSpecificConditionData};
use crate::tacho::{OdometerShort, TimeReal, VUTransferResponseParameterID, VUTransferResponseParameterReader};
use crate::{Readable, Result};

const SIGNATURE_LENGTH: u32 = 128;

/// Data structure generation 1 (TREP 02 Hex)
#[derive(Debug, Serialize)]
pub struct VUActivity {
    #[serde(rename = "dateOfDayDownloaded")]
    pub date_of_day_downloaded: TimeReal,
    #[serde(rename = "odometerValueMidnight")]
    pub odometer_value_midnight: OdometerShort,
    #[serde(rename = "vuCardIWData")]
    pub vu_card_iw_data: VuCardIWData,
    #[serde(rename = "vuActivityDailyData")]
    pub vu_activity_daily_data: VuActivityDailyData,
    #[serde(rename = "vuPlaceDailyWorkPeriodData")]
    pub vu_place_daily_work_period_data: VuPlaceDailyWorkPeriodData,
    #[serde(rename = "vuSpecificConditionData")]
    pub vu_specific_condition_data: VuSpecificConditionData,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VUActivity> for VUActivity {
    fn from_data<R: ReadBytes + BinSeek>(_trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        let date_of_day_downloaded = TimeReal::read(reader)?;
        let odometer_value_midnight = OdometerShort::read(reader)?;
        let vu_card_iw_data = VuCardIWData::read(reader)?;
        let vu_activity_daily_data = VuActivityDailyData::read(reader)?;
        let vu_place_daily_work_period_data = VuPlaceDailyWorkPeriodData::read(reader)?;
        let vu_specific_condition_data = VuSpecificConditionData::read(reader)?;
        let signature: Option<Vec<u8>> = Some(reader.read_into_vec(SIGNATURE_LENGTH)?);

        Ok(Self {
            date_of_day_downloaded,
            odometer_value_midnight,
            vu_card_iw_data,
            vu_activity_daily_data,
            vu_place_daily_work_period_data,
            vu_specific_condition_data,
            signature,
        })
    }
}
