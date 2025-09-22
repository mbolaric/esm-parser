use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, SensorExternalGNSSCoupledRecord, SensorPairedRecord, SignatureRecordArray,
    VuCalibrationRecord, VuCardRecord, VuIdentification, VuItsConsentRecord, VuPowerSupplyInterruptionRecord,
};
use crate::tacho::VUTransferResponseParameterID;

/// Data structure generation 2, version 1 (TREP 25 Hex)
/// Data structure generation 2, version 2 (TREP 35 Hex)
#[derive(Debug, Serialize)]
pub struct VUTechnicalData {
    #[serde(rename = "vuIdentificationRecordArray")]
    pub vu_identification_record_array: DataInfoGenericRecordArray<VuIdentification>,
    #[serde(rename = "vuSensorPairedRecordArray")]
    pub vu_sensor_paired_record_array: DataInfoGenericRecordArray<SensorPairedRecord>,
    #[serde(rename = "vuSensorExternalGnssCoupledRecordArray")]
    pub vu_sensor_external_gnss_coupled_record_array: DataInfoGenericRecordArray<SensorExternalGNSSCoupledRecord>,
    #[serde(rename = "vuCalibrationRecordArray")]
    pub vu_calibration_record_array: DataInfoGenericRecordArray<VuCalibrationRecord>,
    #[serde(rename = "vuCardRecordArray")]
    pub vu_card_record_array: DataInfoGenericRecordArray<VuCardRecord>,
    #[serde(rename = "vuItsConsentRecordArray")]
    pub vu_its_consent_record_array: DataInfoGenericRecordArray<VuItsConsentRecord>,
    #[serde(rename = "vuPowerSupplyInterruptionRecordArray")]
    pub vu_power_supply_interruption_record_array: DataInfoGenericRecordArray<VuPowerSupplyInterruptionRecord>,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUTechnicalData {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUTechnicalData> {
        debug!("VUTechnicalData::from_data - Trep ID: {trep_id:?}");
        let vu_identification_record_array: DataInfoGenericRecordArray<VuIdentification> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_sensor_paired_record_array: DataInfoGenericRecordArray<SensorPairedRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_sensor_external_gnss_coupled_record_array: DataInfoGenericRecordArray<SensorExternalGNSSCoupledRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_calibration_record_array: DataInfoGenericRecordArray<VuCalibrationRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_card_record_array: DataInfoGenericRecordArray<VuCardRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_its_consent_record_array: DataInfoGenericRecordArray<VuItsConsentRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_power_supply_interruption_record_array: DataInfoGenericRecordArray<VuPowerSupplyInterruptionRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            vu_identification_record_array,
            vu_sensor_paired_record_array,
            vu_sensor_external_gnss_coupled_record_array,
            vu_calibration_record_array,
            vu_card_record_array,
            vu_its_consent_record_array,
            vu_power_supply_interruption_record_array,
            signature_record_array,
        })
    }
}
