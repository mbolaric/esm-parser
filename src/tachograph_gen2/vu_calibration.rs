use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::DataInfo;
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUCalibration {}

impl VUCalibration {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUCalibration> {
        debug!("VUCalibration::from_data - Trep ID: {:?}", trep_id);
        let identification = DataInfo::read(reader, trep_id.clone())?;
        let sensor_paired = DataInfo::read(reader, trep_id.clone())?;
        let sensor_external_gnss_coupled = DataInfo::read(reader, trep_id.clone())?;
        let calibration = DataInfo::read(reader, trep_id.clone())?;
        let card = DataInfo::read(reader, trep_id.clone())?;
        let its_consent = DataInfo::read(reader, trep_id.clone())?;
        let power_supply_interruption = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        debug!(
            "VUCalibration::from_data - \n[1] {:?},\n[2] {:?},\n[3] {:?},\n[4] {:?},\n[5] {:?},\n[6] {:?},\n[7] {:?},\n[8] {:?} \n",
            identification,
            sensor_paired,
            sensor_external_gnss_coupled,
            calibration,
            card,
            its_consent,
            power_supply_interruption,
            signature
        );
        Ok(Self {})
    }
}
