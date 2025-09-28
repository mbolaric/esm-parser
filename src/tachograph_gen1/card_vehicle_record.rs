use serde::Serialize;

use crate::{
    BCDString, Readable,
    tacho::{OdometerShort, TimeReal, VehicleRegistrationIdentification, VehicleUse},
};

const VU_DATA_BLOCK_COUNTER_LENGTH: u32 = 2;

/// Information, stored in a driver or workshop card, related to a period of
/// use of a vehicle during a calendar day (Annex 1C requirements 269, 294, 322, and 345).
#[derive(Debug, Serialize)]
pub struct CardVehicleRecord {
    #[serde(rename = "vehicleOdometerBegin")]
    pub vehicle_odometer_begin: OdometerShort,
    #[serde(rename = "vehicleOdometerEnd")]
    pub vehicle_odometer_end: OdometerShort,
    #[serde(rename = "vehicleFirstUse")]
    pub vehicle_first_use: TimeReal,
    #[serde(rename = "vehicleLastUse")]
    pub vehicle_last_use: TimeReal,
    #[serde(rename = "vehicleRegistration")]
    pub vehicle_registration: VehicleRegistrationIdentification,
    #[serde(rename = "vuDataBlockCounter")]
    pub vu_data_block_counter: String,
}

impl Readable<CardVehicleRecord> for CardVehicleRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardVehicleRecord> {
        let vehicle_odometer_begin = OdometerShort::read(reader)?;
        let vehicle_odometer_end = OdometerShort::read(reader)?;
        let vehicle_first_use = TimeReal::read(reader)?;
        let vehicle_last_use = TimeReal::read(reader)?;
        let vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
        let vu_data_block_counter = BCDString::decode(&reader.read_into_vec(VU_DATA_BLOCK_COUNTER_LENGTH)?)?;

        Ok(Self {
            vehicle_odometer_begin,
            vehicle_odometer_end,
            vehicle_first_use,
            vehicle_last_use,
            vehicle_registration,
            vu_data_block_counter,
        })
    }
}

impl VehicleUse for CardVehicleRecord {
    fn get_vehicle_first_use(&self) -> &TimeReal {
        &self.vehicle_first_use
    }

    fn get_vehicle_last_use(&self) -> &TimeReal {
        &self.vehicle_last_use
    }
}
