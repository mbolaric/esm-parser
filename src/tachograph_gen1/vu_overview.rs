use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::gen1::{CompanyLocks, ControlActivity, DownloadActivity, DownloadablePeriod};
use crate::helpers::vec_u8_to_string;
use crate::tacho::{
    CardSlotStatus, TimeReal, VUTransferResponseParameterID, VUTransferResponseParameterReader, VehicleRegistrationIdentification,
};
use crate::{Readable, Result};

#[derive(Debug)]
pub struct VuOverview {
    pub member_state_certificate: Vec<u8>,
    pub vu_certificate: Vec<u8>,
    pub vehicle_identification_number: String,
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub current_date_time: TimeReal,
    pub downloadable_period: DownloadablePeriod,
    pub card_slot_status: CardSlotStatus,
    pub download_activity: DownloadActivity,
    pub company_locks: CompanyLocks,
    pub control_activity: ControlActivity,
    pub signature: Option<Vec<u8>>,
}

impl VUTransferResponseParameterReader<VuOverview> for VuOverview {
    fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VuOverview> {
        debug!("VUControl::from_data - Trep ID: {:?}", trep_id);
        let member_state_certificate = reader.read_into_vec(194)?;
        let vu_certificate = reader.read_into_vec(194)?;
        let vehicle_identification_number = vec_u8_to_string(reader.read_into_vec(17)?)?;
        let vehicle_registration_identification: VehicleRegistrationIdentification =
            VehicleRegistrationIdentification::read(reader)?;
        let current_date_time = TimeReal::read(reader)?;
        let downloadable_period = DownloadablePeriod::read(reader)?;
        let card_slot_status = CardSlotStatus::read(reader)?;
        let download_activity = DownloadActivity::read(reader)?;
        let company_locks = CompanyLocks::read(reader)?;
        let control_activity = ControlActivity::read(reader)?;
        let signature = Some(reader.read_into_vec(128)?);

        Ok(Self {
            member_state_certificate,
            vu_certificate,
            vehicle_identification_number,
            vehicle_registration_identification,
            current_date_time,
            downloadable_period,
            card_slot_status,
            download_activity,
            company_locks,
            control_activity,
            signature,
        })
    }
}
