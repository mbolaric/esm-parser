use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{DataInfo, DataInfoGenericRecords, VuCardIWRecord};
use crate::tacho::{OdometerShort, TimeReal, VUTransferResponseParameterID};

#[derive(Debug)]
pub struct VUActivity {
    pub date_of_day_downloaded_records: DataInfoGenericRecords<TimeReal>,
    pub odometer_value_midnight_records: DataInfoGenericRecords<OdometerShort>,
    pub vu_card_iw_records: DataInfoGenericRecords<VuCardIWRecord>,
}

impl VUActivity {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        debug!("VUControlActivity::from_data - Trep ID: {:?}", trep_id);
        let date_of_day_downloaded_records: DataInfoGenericRecords<TimeReal> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let odometer_value_midnight_records: DataInfoGenericRecords<OdometerShort> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_card_iw_records: DataInfoGenericRecords<VuCardIWRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let activity_change_info = DataInfo::read(reader, trep_id.clone())?;
        let place_daily_work_period = DataInfo::read(reader, trep_id.clone())?;
        let gns_sad = DataInfo::read(reader, trep_id.clone())?;
        let specific_condition = DataInfo::read(reader, trep_id.clone())?;

        if trep_id == VUTransferResponseParameterID::Gen2v2Activities {
            // Two record are not in use
            DataInfo::read(reader, trep_id.clone())?;
            DataInfo::read(reader, trep_id.clone())?;
        }
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self { date_of_day_downloaded_records, odometer_value_midnight_records, vu_card_iw_records })
    }
}
