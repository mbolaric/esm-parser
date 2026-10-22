use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, SignatureRecordArray, VuActivityDailyRecordArray, VuBorderCrossingRecord,
    VuCardIWRecord, VuGnssadRecord, VuLoadUnloadRecord, VuPlaceDailyWorkPeriodRecordArray,
};
use crate::tacho::{OdometerShort, RecordType, SpecificConditionRecord, TimeReal, VUTransferResponseParameterID};
use crate::{Error, Result};

/// Data structure generation 2, version 1 (TREP 22 Hex)
/// Data structure generation 2, version 2 (TREP 32 Hex)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VUActivity"))]
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
    #[serde(rename = "vuBorderCrossingRecordArray")]
    pub vu_border_crossing_record_array: Option<DataInfoGenericRecordArray<VuBorderCrossingRecord>>,
    #[serde(rename = "vuLoadUnloadRecordArray")]
    pub vu_load_unload_record_array: Option<DataInfoGenericRecordArray<VuLoadUnloadRecord>>,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUActivity {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        debug!("VUControlActivity::from_data - Trep ID: {trep_id:?}");
        let date_of_day_downloaded_record_array: DataInfoGenericRecordArray<TimeReal> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let odometer_value_midnight_record_array: DataInfoGenericRecordArray<OdometerShort> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let vu_card_iw_record_array: DataInfoGenericRecordArray<VuCardIWRecord> = DataInfo::read(reader, trep_id)?.parse()?;
        let vu_activity_daily_record_array: VuActivityDailyRecordArray = DataInfo::read(reader, trep_id)?.parse()?;
        let vu_place_daily_work_period_record_array: VuPlaceDailyWorkPeriodRecordArray =
            DataInfo::read(reader, trep_id)?.parse()?;
        let gnss_data = DataInfo::read(reader, trep_id)?;
        let expected_size = if trep_id == VUTransferResponseParameterID::Gen2v2Activities {
            VuGnssadRecord::RECORD_SIZE_V2
        } else {
            VuGnssadRecord::RECORD_SIZE_V1
        };
        if gnss_data.record_size != expected_size {
            return Err(Error::InvalidDataParse(format!(
                "VU GNSS accumulated-driving record size: expected {expected_size} bytes, got {}",
                gnss_data.record_size
            )));
        }
        let vu_gnssad_record_array: DataInfoGenericRecordArray<VuGnssadRecord> = gnss_data.parse_with_params()?;
        let vu_specific_condition_record_array: DataInfoGenericRecordArray<SpecificConditionRecord> =
            DataInfo::read(reader, trep_id)?.parse()?;

        let (vu_border_crossing_record_array, vu_load_unload_record_array) =
            if trep_id == VUTransferResponseParameterID::Gen2v2Activities {
                let borders = DataInfo::read(reader, trep_id)?
                    .parse_record_array(RecordType::VuBorderCrossingRecord, VuBorderCrossingRecord::RECORD_SIZE)?;
                let operations = DataInfo::read(reader, trep_id)?
                    .parse_record_array(RecordType::VuLoadUnloadRecord, VuLoadUnloadRecord::RECORD_SIZE)?;
                (Some(borders), Some(operations))
            } else {
                (None, None)
            };
        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id)?.parse()?);

        Ok(Self {
            date_of_day_downloaded_record_array,
            odometer_value_midnight_record_array,
            vu_card_iw_record_array,
            vu_activity_daily_record_array,
            vu_place_daily_work_period_record_array,
            vu_gnssad_record_array,
            vu_specific_condition_record_array,
            vu_border_crossing_record_array,
            vu_load_unload_record_array,
            signature_record_array,
        })
    }
}
