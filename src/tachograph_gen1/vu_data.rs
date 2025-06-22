use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    gen1::{VUActivity, VUCalibration, VUControl, VUTransferResponseParameterData},
    tacho::{
        self, TachographHeader, VUTransferResponseParameterID, VUTransferResponseParameterItem,
        VUTransferResponseParameterReader,
    },
    tachograph, Result,
};

#[derive(Debug)]
pub struct VUData {
    header: TachographHeader,
    transfer_res_params: Vec<VUTransferResponseParameterItem<VUTransferResponseParameterData>>,
}

impl VUData {
    pub fn from_data<R: ReadBytes + BinSeek>(
        header: TachographHeader,
        reader: &mut R,
    ) -> Result<VUData> {
        let transfer_res_params = <dyn tacho::VUData<VUTransferResponseParameterData>>::from_data(
            reader,
            &|trep_id: VUTransferResponseParameterID, reader: &mut R| {
                VUData::parse_trep(trep_id, reader)
            },
        )?;

        Ok(VUData {
            header,
            transfer_res_params,
        })
    }

    fn parse_trep<R: ReadBytes + BinSeek>(
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUTransferResponseParameterData> {
        debug!("VUData::parse_trep - {:?}", trep_id);
        match trep_id {
            VUTransferResponseParameterID::Control => {
                let vu_control = VUControl::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Control(vu_control))
            }
            VUTransferResponseParameterID::Activity => {
                let vu_activities = VUActivity::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Activity(vu_activities))
            }
            VUTransferResponseParameterID::Events => Ok(VUTransferResponseParameterData::Events),
            VUTransferResponseParameterID::Speed => Ok(VUTransferResponseParameterData::Speed),
            VUTransferResponseParameterID::Calibration => {
                let vu_calibration = VUCalibration::from_data(trep_id, reader)?;
                Ok(VUTransferResponseParameterData::Calibration(vu_calibration))
            }
            VUTransferResponseParameterID::CardDownload => {
                Ok(VUTransferResponseParameterData::CardDownload)
            }
            VUTransferResponseParameterID::OddballCrashDump => {
                Ok(VUTransferResponseParameterData::OddballCrashDump)
            }
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
