use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{DataInfo, VuIdentificationRecords};
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUTechnicalData {
    pub vu_identification_records: VuIdentificationRecords,
}

impl VUTechnicalData {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUTechnicalData> {
        debug!("VUTechnicalData::from_data - Trep ID: {:?}", trep_id);
        let vu_identification_records: VuIdentificationRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;

        let vu_sensor_paired_records = DataInfo::read(reader, trep_id.clone())?;
        let sensor_external_gnss_coupled = DataInfo::read(reader, trep_id.clone())?;
        let calibration = DataInfo::read(reader, trep_id.clone())?;
        let card = DataInfo::read(reader, trep_id.clone())?;
        let its_consent = DataInfo::read(reader, trep_id.clone())?;
        let power_supply_interruption = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self { vu_identification_records })
    }
}
