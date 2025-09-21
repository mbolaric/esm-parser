use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::{
    Result,
    gen2::{
        DataInfo, DataInfoGenericRecordArray, MemberStateCertificateRecordArray, SignatureRecordArray,
        VehicleIdentificationNumberRecordArray, VehicleRegistrationNumberRecordArray, VuCertificateRecordArray,
        VuCompanyLocksRecord, VuControlActivityRecord, VuDownloadActivityData, VuDownloadablePeriod,
    },
    tacho::{CardSlotStatus, TimeReal, VUTransferResponseParameterID},
    tachograph_gen2::vehicle_registration_identification_records::VehicleRegistrationIdentificationRecords,
};

/// Data structure generation 2, version 2 (TREP 31 Hex)
#[derive(Debug, Serialize)]
pub struct VUOverview {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    #[serde(rename = "nemberStateCertificateRecordArray")]
    pub member_state_certificate_record_array: MemberStateCertificateRecordArray,
    #[serde(rename = "vuCertificateRecordArray")]
    pub vu_certificate_record_array: VuCertificateRecordArray,
    #[serde(rename = "vehicleIdentificationNumberRecordArray")]
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray,
    #[serde(rename = "vehicleRegistrationNumberRecordArray")]
    pub vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArray,
    #[serde(rename = "CurrentDateTimeRecordArray")]
    pub current_date_time_record_array: DataInfoGenericRecordArray<TimeReal>,
    #[serde(rename = "vuDownloadablePeriodRecordArray")]
    pub vu_downloadale_period_record_array: DataInfoGenericRecordArray<VuDownloadablePeriod>,
    #[serde(rename = "cardSlotsStatusRecordArray")]
    pub card_slot_status_record_array: DataInfoGenericRecordArray<CardSlotStatus>,
    #[serde(rename = "vuDownloadActivityDataRecordArray")]
    pub vu_download_activity_data_record_array: DataInfoGenericRecordArray<VuDownloadActivityData>,
    #[serde(rename = "vuCompanyLocksRecordArray")]
    pub vu_company_locks_record_array: DataInfoGenericRecordArray<VuCompanyLocksRecord>,
    #[serde(rename = "vuControlActivityRecordArray")]
    pub vu_control_activity_record_array: DataInfoGenericRecordArray<VuControlActivityRecord>,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: SignatureRecordArray,
}

impl VUOverview {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUOverview> {
        debug!("VUControl::from_data - Trep ID: {trep_id:?}");
        let member_state_certificate_record_array: MemberStateCertificateRecordArray =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_certificate_record_array: VuCertificateRecordArray = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        let vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArray =
            if trep_id == VUTransferResponseParameterID::Gen2v2Overview {
                let records: VehicleRegistrationIdentificationRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
                VehicleRegistrationNumberRecordArray::from(records)
            } else {
                DataInfo::read(reader, trep_id.clone())?.parse()?
            };

        let current_date_time_record_array: DataInfoGenericRecordArray<TimeReal> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_downloadale_period_record_array: DataInfoGenericRecordArray<VuDownloadablePeriod> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let card_slot_status_record_array: DataInfoGenericRecordArray<CardSlotStatus> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_download_activity_data_record_array: DataInfoGenericRecordArray<VuDownloadActivityData> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_company_locks_record_array: DataInfoGenericRecordArray<VuCompanyLocksRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_control_activity_record_array: DataInfoGenericRecordArray<VuControlActivityRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_record_array: SignatureRecordArray = DataInfo::read(reader, trep_id.clone())?.parse()?;

        Ok(Self {
            trep_id,
            member_state_certificate_record_array,
            vu_certificate_record_array,
            vehicle_identification_number_record_array,
            vehicle_registration_number_record_array,
            current_date_time_record_array,
            vu_downloadale_period_record_array,
            card_slot_status_record_array,
            vu_download_activity_data_record_array,
            vu_company_locks_record_array,
            vu_control_activity_record_array,
            signature_record_array,
        })
    }
}
