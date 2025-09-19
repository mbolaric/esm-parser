use serde::Serialize;

use crate::{
    Readable,
    tacho::{ControlType, FullCardNumber, TimeReal, VehicleRegistrationIdentification},
};

/// Information, stored in a control card, related to control activity
/// performed with the card (Annex 1C requirement 361 and 367).
#[derive(Debug, Serialize)]
pub struct ControlCardActivityRecord {
    #[serde(rename = "controlType")]
    pub control_type: ControlType,
    #[serde(rename = "controlTime")]
    pub control_time: TimeReal,
    #[serde(rename = "controlledCardNumber")]
    pub controlled_card_number: FullCardNumber,
    #[serde(rename = "controlledVehicleRegistration")]
    pub controlled_vehicle_registration: VehicleRegistrationIdentification,
    #[serde(rename = "controlDownloadPeriodBegin")]
    pub control_download_period_begin: TimeReal,
    #[serde(rename = "controlDownloadPeriodEnd")]
    pub control_download_period_end: TimeReal,
}

impl Readable<ControlCardActivityRecord> for ControlCardActivityRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<ControlCardActivityRecord> {
        let control_type: ControlType = reader.read_u8()?.into();
        let control_time = TimeReal::read(reader)?;
        let controlled_card_number = FullCardNumber::read(reader)?;
        let controlled_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
        let control_download_period_begin = TimeReal::read(reader)?;
        let control_download_period_end = TimeReal::read(reader)?;

        Ok(Self {
            control_type,
            control_time,
            controlled_card_number,
            controlled_vehicle_registration,
            control_download_period_begin,
            control_download_period_end,
        })
    }
}
