use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecords, SensorExternalGNSSCoupledRecord, SensorPairedRecord, VuCalibrationRecord, VuCardRecord,
    VuIdentification,
};
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug)]
pub struct VUTechnicalData {
    pub vu_identification_records: DataInfoGenericRecords<VuIdentification>,
    pub vu_sensor_paired_records: DataInfoGenericRecords<SensorPairedRecord>,
    pub vu_sensor_external_gnss_coupled_records: DataInfoGenericRecords<SensorExternalGNSSCoupledRecord>,
    pub vu_calibration_records: DataInfoGenericRecords<VuCalibrationRecord>,
    pub vu_card_records: DataInfoGenericRecords<VuCardRecord>,
}

impl VUTechnicalData {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUTechnicalData> {
        debug!("VUTechnicalData::from_data - Trep ID: {:?}", trep_id);
        let vu_identification_records: DataInfoGenericRecords<VuIdentification> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_sensor_paired_records: DataInfoGenericRecords<SensorPairedRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_sensor_external_gnss_coupled_records: DataInfoGenericRecords<SensorExternalGNSSCoupledRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_calibration_records: DataInfoGenericRecords<VuCalibrationRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_card_records: DataInfoGenericRecords<VuCardRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;

        // FIXME:
        let its_consent = DataInfo::read(reader, trep_id.clone())?;
        let power_supply_interruption = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self {
            vu_identification_records,
            vu_sensor_paired_records,
            vu_sensor_external_gnss_coupled_records,
            vu_calibration_records,
            vu_card_records,
        })
    }
}
