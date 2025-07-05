use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::DataInfo;
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUActivity {}

impl VUActivity {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUActivity> {
        debug!("VUControlActivity::from_data - Trep ID: {:?}", trep_id);
        let date_of_day_downloaded = DataInfo::read(reader, trep_id.clone())?;
        let odometer_value_midnight = DataInfo::read(reader, trep_id.clone())?;
        let card_iw = DataInfo::read(reader, trep_id.clone())?;
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

        Ok(Self {})
    }
}
