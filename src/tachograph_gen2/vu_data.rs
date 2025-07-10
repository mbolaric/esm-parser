use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    Result,
    gen2::{DataInfo, VUActivity, VUCalibration, VUCardDownload, VUControl, VUEvents, VUSpeed, VUTransferResponseParameterData},
    tacho::{self, TachographHeader, VUTransferResponseParameterID, VUTransferResponseParameterItem},
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

    fn parse_speed<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_speed - Trep ID: {:?}", trep_id);
        let vu_speed = VUSpeed::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::Speed(vu_speed))
    }

    fn parse_control<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_control - Trep ID: {:?}", trep_id);
        let vu_control = VUControl::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::Control(vu_control))
    }

    fn parse_activity<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_activity - Trep ID: {:?}", trep_id);
        let vu_control_activity = VUActivity::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::Activity(vu_control_activity))
    }

    fn parse_events<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_events - Trep ID: {:?}", trep_id);
        let vu_events = VUEvents::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::Events(vu_events))
    }

    fn parse_calibration<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_calibration - Trep ID: {:?}", trep_id);
        let vu_calibration = VUCalibration::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::Calibration(vu_calibration))
    }

    fn parse_card_download<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_card_download - Trep ID: {:?}", trep_id);
        let vu_card_download = VUCardDownload::from_data(trep_id, reader)?;
        Ok(VUTransferResponseParameterData::CardDownload(vu_card_download))
    }

    fn parse_trep<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_trep - Trep ID: {:?}", trep_id);
        match trep_id {
            VUTransferResponseParameterID::Overview
            | VUTransferResponseParameterID::Gen2Overview
            | VUTransferResponseParameterID::Gen2v2Overview => VUData::parse_control(trep_id, reader),
            VUTransferResponseParameterID::Activities
            | VUTransferResponseParameterID::Gen2Activities
            | VUTransferResponseParameterID::Gen2v2Activities => VUData::parse_activity(trep_id, reader),
            VUTransferResponseParameterID::EventsAndFaults
            | VUTransferResponseParameterID::Gen2EventsAndFaults
            | VUTransferResponseParameterID::Gen2v2EventsAndFaults => VUData::parse_events(trep_id, reader),
            VUTransferResponseParameterID::TechnicalData
            | VUTransferResponseParameterID::Gen2TechnicalData
            | VUTransferResponseParameterID::Gen2v2TechnicalData => VUData::parse_calibration(trep_id, reader),
            VUTransferResponseParameterID::Speed
            | VUTransferResponseParameterID::Gen2Speed
            | VUTransferResponseParameterID::Gen2v2Speed => VUData::parse_speed(trep_id, reader),
            VUTransferResponseParameterID::CardDownload => VUData::parse_card_download(trep_id, reader),
            VUTransferResponseParameterID::OddballCrashDump => Ok(VUTransferResponseParameterData::OddballCrashDump),
            _ => {
                let data_info = DataInfo::read(reader, trep_id)?;
                debug!("VUData::parse_trep - Not Imeplemented {:?}", data_info);
                Ok(VUTransferResponseParameterData::Unknown(data_info))
            }
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
