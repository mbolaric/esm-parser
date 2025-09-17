use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::{
    Result,
    gen2::{
        DataInfo, DataInfoGenericRecords, MemberStateCertificateRecords, SignatureRecords, VehicleIdentificationNumberRecords,
        VuCertificateRecords, VuCompanyLocksRecord, VuControlActivityRecord, VuDownloadActivityData, VuDownloadablePeriod,
        VuVehicleRegistrationNumberRecords,
    },
    tacho::{CardSlotStatus, TimeReal, VUTransferResponseParameterID},
    tachograph_gen2::vehicle_registration_identification_records::VehicleRegistrationIdentificationRecords,
};

#[derive(Debug, Serialize)]
pub struct VUOverview {
    pub trep_id: VUTransferResponseParameterID,
    pub member_state_certificate_records: MemberStateCertificateRecords,
    pub vu_certificate_records: VuCertificateRecords,
    pub vehicle_identification_number_records: VehicleIdentificationNumberRecords,
    pub vu_vehicle_registration_number_records: VuVehicleRegistrationNumberRecords,
    pub current_date_time_records: DataInfoGenericRecords<TimeReal>,
    pub vu_downloadale_period_records: DataInfoGenericRecords<VuDownloadablePeriod>,
    pub card_slot_status_records: DataInfoGenericRecords<CardSlotStatus>,
    pub vu_download_activity_data_records: DataInfoGenericRecords<VuDownloadActivityData>,
    pub vu_company_locks_records: DataInfoGenericRecords<VuCompanyLocksRecord>,
    pub vu_control_activity_records: DataInfoGenericRecords<VuControlActivityRecord>,
    pub signature_records: SignatureRecords,
}

impl VUOverview {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUOverview> {
        debug!("VUControl::from_data - Trep ID: {trep_id:?}");
        let member_state_certificate_records: MemberStateCertificateRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_certificate_records: VuCertificateRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vehicle_identification_number_records: VehicleIdentificationNumberRecords =
            DataInfo::read(reader, trep_id.clone())?.parse()?;

        let vu_vehicle_registration_number_records: VuVehicleRegistrationNumberRecords =
            if trep_id == VUTransferResponseParameterID::Gen2v2Overview {
                let records: VehicleRegistrationIdentificationRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
                VuVehicleRegistrationNumberRecords::from(records)
            } else {
                DataInfo::read(reader, trep_id.clone())?.parse()?
            };

        let current_date_time_records: DataInfoGenericRecords<TimeReal> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_downloadale_period_records: DataInfoGenericRecords<VuDownloadablePeriod> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let card_slot_status_records: DataInfoGenericRecords<CardSlotStatus> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_download_activity_data_records: DataInfoGenericRecords<VuDownloadActivityData> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_company_locks_records: DataInfoGenericRecords<VuCompanyLocksRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_control_activity_records: DataInfoGenericRecords<VuControlActivityRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_records: SignatureRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;

        Ok(Self {
            trep_id,
            member_state_certificate_records,
            vu_certificate_records,
            vehicle_identification_number_records,
            vu_vehicle_registration_number_records,
            current_date_time_records,
            vu_downloadale_period_records,
            card_slot_status_records,
            vu_download_activity_data_records,
            vu_company_locks_records,
            vu_control_activity_records,
            signature_records,
        })
    }
}
