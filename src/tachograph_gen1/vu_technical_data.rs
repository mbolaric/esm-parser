use binary_data::{BinSeek, ReadBytes};

use crate::gen1::{SensorPaired, VUCalibrationData, VUIdentification};
use crate::tacho::{VUTransferResponseParameterID, VUTransferResponseParameterReader};
use crate::{Readable, Result};

#[derive(Debug)]
pub struct VuTechnicalData {
    pub trep_id: VUTransferResponseParameterID,
    pub identification: VUIdentification,
    pub sensor_paired: SensorPaired,
    pub calibration_data: VUCalibrationData,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VuTechnicalData> for VuTechnicalData {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VuTechnicalData> {
        let vu_identification = VUIdentification::read(reader)?;
        let sensor_paired = SensorPaired::read(reader)?;
        let vu_calibration_data = VUCalibrationData::read(reader)?;

        let signature = Some(reader.read_into_vec(128)?);

        Ok(Self { trep_id, identification: vu_identification, sensor_paired, calibration_data: vu_calibration_data, signature })
    }
}
