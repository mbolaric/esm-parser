use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    Readable, Result,
    gen1::{VUActivity, VUTransferResponseParameterData, VuDetailedSpeed, VuEvents, VuOverview, VuTechnicalData},
    tacho::{
        self, TachographHeader, VUTransferResponseParameterID, VUTransferResponseParameterItem, VUTransferResponseParameterReader,
    },
    tachograph,
};

#[derive(Debug)]
pub struct VUData {
    header: TachographHeader,
    transfer_res_params: Vec<VUTransferResponseParameterItem<VUTransferResponseParameterData>>,
}

impl VUData {
    pub fn from_data<R: ReadBytes + BinSeek>(header: TachographHeader, reader: &mut R) -> Result<VUData> {
        let transfer_res_params = <dyn tacho::VUData<VUTransferResponseParameterData>>::from_data(
            reader,
            &|trep_id: VUTransferResponseParameterID, reader: &mut R| VUData::parse_trep(trep_id, reader),
        )?;

        Ok(VUData { header, transfer_res_params })
    }

    fn parse_trep<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_trep - Trep ID: {:?}", trep_id);
        match trep_id {
            VUTransferResponseParameterID::Overview => {
                let vu_control = VuOverview::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Control(vu_control))
            }
            VUTransferResponseParameterID::Activities => {
                let vu_activities = VUActivity::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Activity(vu_activities))
            }
            VUTransferResponseParameterID::EventsAndFaults => {
                let vu_events = VuEvents::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Events(vu_events))
            }
            VUTransferResponseParameterID::Speed => {
                let vu_speed = VuDetailedSpeed::read(reader)?;
                Ok(VUTransferResponseParameterData::Speed(vu_speed))
            }
            VUTransferResponseParameterID::TechnicalData => {
                let vu_calibration = VuTechnicalData::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Calibration(vu_calibration))
            }
            VUTransferResponseParameterID::CardDownload => Ok(VUTransferResponseParameterData::CardDownload),
            VUTransferResponseParameterID::OddballCrashDump => Ok(VUTransferResponseParameterData::OddballCrashDump),
            _ => Ok(VUTransferResponseParameterData::Unknown),
        }
    }
}

impl tachograph::VUData<VUTransferResponseParameterData> for VUData {
    fn get_header(&self) -> &TachographHeader {
        &self.header
    }

    fn get_data(&self) -> &Vec<VUTransferResponseParameterItem<VUTransferResponseParameterData>> {
        &self.transfer_res_params
    }
}
