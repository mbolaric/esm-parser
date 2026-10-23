use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, MemberStateCertificateRecordArray, SignatureRecordArray,
    VehicleIdentificationNumberRecordArray, VehicleRegistrationIdentificationRecordArray, VehicleRegistrationNumberRecordArray,
    VuCertificateRecordArray, VuCompanyLocksRecord, VuControlActivityRecord, VuDownloadActivityData, VuDownloadablePeriod,
};
use crate::tacho::{CardSlotStatus, TimeReal, VUTransferResponseParameterID};

/// Data structure generation 2, version 1 (TREP 21 Hex)
/// Data structure generation 2, version 2 (TREP 31 Hex)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VUOverview"))]
pub struct VUOverview {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    #[serde(rename = "memberStateCertificateRecordArray")]
    pub member_state_certificate_record_array: MemberStateCertificateRecordArray,
    #[serde(rename = "memberStateCertificateRaw")]
    pub member_state_certificate_raw: Vec<u8>,
    #[serde(rename = "vuCertificateRecordArray")]
    pub vu_certificate_record_array: VuCertificateRecordArray,
    #[serde(rename = "vuCertificateRaw")]
    pub vu_certificate_raw: Vec<u8>,
    #[serde(rename = "vehicleIdentificationNumberRecordArray")]
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray,
    /// Generation 2, version 1 (TREP 21 Hex).
    #[serde(rename = "vehicleRegistrationNumberRecordArray")]
    pub vehicle_registration_number_record_array: Option<VehicleRegistrationNumberRecordArray>,
    /// Generation 2, version 2 (TREP 31 Hex).
    #[serde(rename = "vehicleRegistrationIdentificationRecordArray")]
    pub vehicle_registration_identification_record_array: Option<VehicleRegistrationIdentificationRecordArray>,
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
        let member_state_certificate_data_info = DataInfo::read(reader, trep_id)?;
        let member_state_certificate_record_array: MemberStateCertificateRecordArray =
            member_state_certificate_data_info.parse()?;
        let vu_certificate_data_info = DataInfo::read(reader, trep_id)?;
        let vu_certificate_record_array: VuCertificateRecordArray = vu_certificate_data_info.parse()?;
        let vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray =
            DataInfo::read(reader, trep_id)?.parse()?;

        let registration = DataInfo::read(reader, trep_id)?;
        let (vehicle_registration_number_record_array, vehicle_registration_identification_record_array) =
            if trep_id == VUTransferResponseParameterID::Gen2v2Overview {
                (None, Some(registration.parse()?))
            } else {
                (Some(registration.parse()?), None)
            };

        let current_date_time_record_array: DataInfoGenericRecordArray<TimeReal> = DataInfo::read(reader, trep_id)?.parse()?;
        let vu_downloadale_period_record_array: DataInfoGenericRecordArray<VuDownloadablePeriod> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let card_slot_status_record_array: DataInfoGenericRecordArray<CardSlotStatus> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let vu_download_activity_data_record_array: DataInfoGenericRecordArray<VuDownloadActivityData> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let vu_company_locks_record_array: DataInfoGenericRecordArray<VuCompanyLocksRecord> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let vu_control_activity_record_array: DataInfoGenericRecordArray<VuControlActivityRecord> =
            DataInfo::read(reader, trep_id)?.parse()?;
        let signature_record_array: SignatureRecordArray = DataInfo::read(reader, trep_id)?.parse()?;

        Ok(Self {
            trep_id,
            member_state_certificate_record_array,
            member_state_certificate_raw: member_state_certificate_data_info.data,
            vu_certificate_record_array,
            vu_certificate_raw: vu_certificate_data_info.data,
            vehicle_identification_number_record_array,
            vehicle_registration_number_record_array,
            vehicle_registration_identification_record_array,
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
