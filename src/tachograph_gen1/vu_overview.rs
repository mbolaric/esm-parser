use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::gen1::{VuCompanyLocksData, VuControlActivity, VuDownloadActivityData, VuDownloadablePeriod};
use crate::tacho::{
    CardSlotStatus, TimeReal, VUTransferResponseParameterID, VUTransferResponseParameterReader, VehicleRegistrationIdentification,
};
use crate::{Readable, Result, bytes_to_ia5_fix_string};

/// Data Overview
#[derive(Debug, Serialize)]
pub struct VuOverview {
    #[serde(rename = "memberStateCertificate")]
    pub member_state_certificate: Vec<u8>,
    #[serde(rename = "vuCertificate")]
    pub vu_certificate: Vec<u8>,
    /// Vehicle Identification Number (VIN) referring to the vehicle as a whole,
    /// normally chassis serial number or frame number.
    #[serde(rename = "vehicleIdentificationNumber")]
    pub vehicle_identification_number: String,
    /// Identification of a vehicle, unique for Europe (VRN and Member State).
    #[serde(rename = "vehicleRegistrationIdentification")]
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    #[serde(rename = "currentDateTime")]
    pub current_date_time: TimeReal,
    #[serde(rename = "vuDownloadablePeriod")]
    pub vu_downloadable_period: VuDownloadablePeriod,
    #[serde(rename = "cardSlotStatus")]
    pub card_slot_status: CardSlotStatus,
    #[serde(rename = "vuDownloadActivityData")]
    pub vu_download_activity_data: VuDownloadActivityData,
    #[serde(rename = "vuCompanyLocksData")]
    pub vu_company_locks_data: VuCompanyLocksData,
    #[serde(rename = "vuControlActivity")]
    pub vu_control_activity: VuControlActivity,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VuOverview> for VuOverview {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VuOverview> {
        debug!("VuOverview::from_data - Trep ID: {trep_id:?}");
        let member_state_certificate = reader.read_into_vec(194)?;
        let vu_certificate = reader.read_into_vec(194)?;
        let vehicle_identification_number = bytes_to_ia5_fix_string(&reader.read_into_vec(17)?)?;
        let vehicle_registration_identification: VehicleRegistrationIdentification =
            VehicleRegistrationIdentification::read(reader)?;
        let current_date_time = TimeReal::read(reader)?;
        let vu_downloadable_period = VuDownloadablePeriod::read(reader)?;
        let card_slot_status = CardSlotStatus::read(reader)?;
        let vu_download_activity_data = VuDownloadActivityData::read(reader)?;
        let vu_company_locks_data = VuCompanyLocksData::read(reader)?;
        let vu_control_activity = VuControlActivity::read(reader)?;
        let signature = Some(reader.read_into_vec(128)?);

        Ok(Self {
            member_state_certificate,
            vu_certificate,
            vehicle_identification_number,
            vehicle_registration_identification,
            current_date_time,
            vu_downloadable_period,
            card_slot_status,
            vu_download_activity_data,
            vu_company_locks_data,
            vu_control_activity,
            signature,
        })
    }
}
