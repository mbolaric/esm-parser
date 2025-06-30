use crate::{
    Readable,
    tacho::{ControlType, FullCardNumber, TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct CardControlActivityData {
    pub control_type: ControlType,
    pub control_time: TimeReal,
    pub control_card_number: FullCardNumber,
    pub control_vehicle_registration: VehicleRegistrationIdentification,
    pub control_download_period_begin: TimeReal,
    pub control_download_period_end: TimeReal,
}

impl Readable<CardControlActivityData> for CardControlActivityData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardControlActivityData> {
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
