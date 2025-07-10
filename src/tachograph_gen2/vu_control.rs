use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::{
    Result,
    gen2::{DataInfo, MemberStateCertificateRecords, VUActivity, VuCertificateRecords},
    tacho::VUTransferResponseParameterID,
};

#[derive(Debug)]
pub struct VUControl {
    pub trep_id: VUTransferResponseParameterID,
    pub member_state_certificate_records: MemberStateCertificateRecords,
    pub vu_certificate_records: VuCertificateRecords,
    pub activities: Vec<VUActivity>,
}

impl VUControl {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUControl> {
        debug!("VUControl::from_data - Trep ID: {:?}", trep_id);
        let member_state_certificate_records: MemberStateCertificateRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_certificate_records: VuCertificateRecords = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vehicle_identification_number = DataInfo::read(reader, trep_id.clone())?;

        let vehicle_registration_identification = if trep_id == VUTransferResponseParameterID::Gen2v2Overview {
            // FIXME:
            DataInfo::read(reader, trep_id.clone())?
        } else {
            DataInfo::read(reader, trep_id.clone())?
        };

        let current_date_time = DataInfo::read(reader, trep_id.clone())?;
        let downloadale_period = DataInfo::read(reader, trep_id.clone())?;
        let card_slot_status = DataInfo::read(reader, trep_id.clone())?;
        let download_activity_data = DataInfo::read(reader, trep_id.clone())?;
        let company_locks = DataInfo::read(reader, trep_id.clone())?;
        let control_activity = DataInfo::read(reader, trep_id.clone())?;
        let signature = Some(DataInfo::read(reader, trep_id.clone())?);

        Ok(Self { trep_id, member_state_certificate_records, vu_certificate_records, activities: Vec::new() })
    }
}
