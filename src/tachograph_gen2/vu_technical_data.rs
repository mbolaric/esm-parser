use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, SensorExternalGNSSCoupledRecord, SensorPairedRecord, SignatureRecordArray,
    VuCalibrationRecord, VuCardRecord, VuIdentification, VuItsConsentRecord, VuPowerSupplyInterruptionRecord,
};
use crate::tacho::VUTransferResponseParameterID;

#[derive(Debug, Serialize)]
pub struct VUTechnicalData {
    pub vu_identification_records: DataInfoGenericRecordArray<VuIdentification>,
    pub vu_sensor_paired_records: DataInfoGenericRecordArray<SensorPairedRecord>,
    pub vu_sensor_external_gnss_coupled_records: DataInfoGenericRecordArray<SensorExternalGNSSCoupledRecord>,
    pub vu_calibration_records: DataInfoGenericRecordArray<VuCalibrationRecord>,
    pub vu_card_records: DataInfoGenericRecordArray<VuCardRecord>,
    pub vu_its_consent_records: DataInfoGenericRecordArray<VuItsConsentRecord>,
    pub vu_power_supply_interruption_records: DataInfoGenericRecordArray<VuPowerSupplyInterruptionRecord>,
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUTechnicalData {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUTechnicalData> {
        debug!("VUTechnicalData::from_data - Trep ID: {trep_id:?}");
        let vu_identification_records: DataInfoGenericRecordArray<VuIdentification> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_sensor_paired_records: DataInfoGenericRecordArray<SensorPairedRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_sensor_external_gnss_coupled_records: DataInfoGenericRecordArray<SensorExternalGNSSCoupledRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_calibration_records: DataInfoGenericRecordArray<VuCalibrationRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse_with_params()?;
        let vu_card_records: DataInfoGenericRecordArray<VuCardRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_its_consent_records: DataInfoGenericRecordArray<VuItsConsentRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_power_supply_interruption_records: DataInfoGenericRecordArray<VuPowerSupplyInterruptionRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            vu_identification_records,
            vu_sensor_paired_records,
            vu_sensor_external_gnss_coupled_records,
            vu_calibration_records,
            vu_card_records,
            vu_its_consent_records,
            vu_power_supply_interruption_records,
            signature_record_array,
        })
    }
}
