use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecords, SignatureRecords, VuActivityDailyRecords, VuCardIWRecord, VuGnssadRecord,
    VuPlaceDailyWorkPeriodRecords,
};
use crate::tacho::{OdometerShort, SpecificConditionRecord, TimeReal, VUTransferResponseParameterID};

#[derive(Debug)]
pub struct VUActivity {
    pub date_of_day_downloaded_records: DataInfoGenericRecords<TimeReal>,
    pub odometer_value_midnight_records: DataInfoGenericRecords<OdometerShort>,
    pub vu_card_iw_records: DataInfoGenericRecords<VuCardIWRecord>,
    pub vu_activity_daily_records: VuActivityDailyRecords,
    pub place_daily_work_period_records: VuPlaceDailyWorkPeriodRecords,
    pub gnssad_records: DataInfoGenericRecords<VuGnssadRecord>,
    pub specific_condition_records: DataInfoGenericRecords<SpecificConditionRecord>,
    pub signature_records: Option<SignatureRecords>,
}

impl VUActivity {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        debug!("VUControlActivity::from_data - Trep ID: {:?}", trep_id);
        let date_of_day_downloaded_records: DataInfoGenericRecords<TimeReal> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let odometer_value_midnight_records: DataInfoGenericRecords<OdometerShort> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_card_iw_records: DataInfoGenericRecords<VuCardIWRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_activity_daily_records: VuActivityDailyRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let place_daily_work_period_records: VuPlaceDailyWorkPeriodRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let gnssad_records: DataInfoGenericRecords<VuGnssadRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let specific_condition_records: DataInfoGenericRecords<SpecificConditionRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        if trep_id == VUTransferResponseParameterID::Gen2v2Activities {
            // Two record are not in use
            DataInfo::read(reader, trep_id.clone())?;
            DataInfo::read(reader, trep_id.clone())?;
        }
        let signature_records: Option<SignatureRecords> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            date_of_day_downloaded_records,
            odometer_value_midnight_records,
            vu_card_iw_records,
            vu_activity_daily_records,
            place_daily_work_period_records,
            gnssad_records,
            specific_condition_records,
            signature_records,
        })
    }
}
