use serde::Serialize;

use crate::{
    Readable,
    tacho::{ControlType, FullCardNumber, TimeReal, VehicleRegistrationIdentification},
};

/// Information, stored in a driver or workshop card, related to the last
/// control the driver has been subject to (Annex 1C requirements 274,
/// 299, 327, and 350).
#[derive(Debug, Serialize)]
pub struct CardControlActivityDataRecord {
    #[serde(rename = "controlType")]
    pub control_type: ControlType,
    #[serde(rename = "controlTime")]
    pub control_time: TimeReal,
    /// The FullCardNumber of the control officer having performed the control.
    #[serde(rename = "controlCardNumber")]
    pub control_card_number: FullCardNumber,
    /// The VRN and registering Member State of the vehicle in which the control happened.
    #[serde(rename = "controlVehicleRegistration")]
    pub control_vehicle_registration: VehicleRegistrationIdentification,
    #[serde(rename = "controlDownloadPeriodBegin")]
    pub control_download_period_begin: TimeReal,
    #[serde(rename = "controlDownloadPeriodEnd")]
    pub control_download_period_end: TimeReal,
}

impl Readable<CardControlActivityDataRecord> for CardControlActivityDataRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardControlActivityDataRecord> {
        let control_type: ControlType = reader.read_u8()?.into();
        let control_time = TimeReal::read(reader)?;
        let control_card_number = FullCardNumber::read(reader)?;
        let control_vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
        let control_download_period_begin = TimeReal::read(reader)?;
        let control_download_period_end = TimeReal::read(reader)?;
        Ok(Self {
            control_type,
            control_time,
            control_card_number,
            control_vehicle_registration,
            control_download_period_begin,
            control_download_period_end,
        })
    }
}
