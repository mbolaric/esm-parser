use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, SignatureRecordArray, VuActivityDailyRecordArray, VuCardIWRecord, VuGnssadRecord,
    VuPlaceDailyWorkPeriodRecordArray,
};
use crate::tacho::{OdometerShort, SpecificConditionRecord, TimeReal, VUTransferResponseParameterID};

/// Data structure generation 2, version 1 (TREP 22 Hex)
#[derive(Debug, Serialize)]
pub struct VUActivity {
    #[serde(rename = "dateOfDayDownloadedRecordArray")]
    pub date_of_day_downloaded_record_array: DataInfoGenericRecordArray<TimeReal>,
    #[serde(rename = "odometerValueMidnightRecordArray")]
    pub odometer_value_midnight_record_array: DataInfoGenericRecordArray<OdometerShort>,
    #[serde(rename = "vuCardIWRecordArray")]
    pub vu_card_iw_record_array: DataInfoGenericRecordArray<VuCardIWRecord>,
    #[serde(rename = "vuActivityDailyRecordArray")]
    pub vu_activity_daily_record_array: VuActivityDailyRecordArray,
    #[serde(rename = "vuPlaceDailyWorkPeriodRecordArray")]
    pub vu_place_daily_work_period_record_array: VuPlaceDailyWorkPeriodRecordArray,
    #[serde(rename = "vuGnssadRecordArray")]
    pub vu_gnssad_record_array: DataInfoGenericRecordArray<VuGnssadRecord>,
    #[serde(rename = "vuSpecificConditionRecordArray")]
    pub vu_specific_condition_record_array: DataInfoGenericRecordArray<SpecificConditionRecord>,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUActivity {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        debug!("VUControlActivity::from_data - Trep ID: {trep_id:?}");
        let date_of_day_downloaded_record_array: DataInfoGenericRecordArray<TimeReal> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let odometer_value_midnight_record_array: DataInfoGenericRecordArray<OdometerShort> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_card_iw_record_array: DataInfoGenericRecordArray<VuCardIWRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_activity_daily_record_array: VuActivityDailyRecordArray = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_place_daily_work_period_record_array: VuPlaceDailyWorkPeriodRecordArray =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_gnssad_record_array: DataInfoGenericRecordArray<VuGnssadRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_specific_condition_record_array: DataInfoGenericRecordArray<SpecificConditionRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        if trep_id == VUTransferResponseParameterID::Gen2v2Activities {
            // Two record are not in use
            DataInfo::read(reader, trep_id.clone())?;
            DataInfo::read(reader, trep_id.clone())?;
        }
        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            date_of_day_downloaded_record_array,
            odometer_value_midnight_record_array,
            vu_card_iw_record_array,
            vu_activity_daily_record_array,
            vu_place_daily_work_period_record_array,
            vu_gnssad_record_array,
            vu_specific_condition_record_array,
            signature_record_array,
        })
    }
}
