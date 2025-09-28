use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen1::{SensorPaired, VUCalibrationData, VUIdentification};
use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader};
use crate::{Readable, Result};

const SIGNATURE_LENGTH: u32 = 128;

#[derive(Debug, Serialize)]
pub struct VuTechnicalData {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    pub identification: VUIdentification,
    #[serde(rename = "sensorPaired")]
    pub sensor_paired: SensorPaired,
    #[serde(rename = "vuCalibrationData")]
    pub vu_calibration_data: VUCalibrationData,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VuTechnicalData> for VuTechnicalData {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VuTechnicalData> {
        let vu_identification = VUIdentification::read(reader)?;
        let sensor_paired = SensorPaired::read(reader)?;
        let vu_calibration_data = VUCalibrationData::read(reader)?;

        let signature = Some(reader.read_into_vec(SIGNATURE_LENGTH)?);

        Ok(Self { trep_id, identification: vu_identification, sensor_paired, vu_calibration_data, signature })
    }
}
