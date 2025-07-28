use crate::{
    BCDString, Readable,
    tacho::{OdometerShort, TimeReal, VehicleRegistrationIdentification, VehicleUse},
};

#[derive(Debug)]
pub struct CardVehicleRecord {
    pub vehicle_odometer_begin: OdometerShort,
    pub vehicle_odometer_end: OdometerShort,
    pub vehicle_first_use: TimeReal,
    pub vehicle_last_use: TimeReal,
    pub vehicle_registration: VehicleRegistrationIdentification,
    pub vu_data_block_counter: String,
}

impl Readable<CardVehicleRecord> for CardVehicleRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<CardVehicleRecord> {
        let vehicle_odometer_begin = OdometerShort::read(reader)?;
        let vehicle_odometer_end = OdometerShort::read(reader)?;
        let vehicle_first_use = TimeReal::read(reader)?;
        let vehicle_last_use = TimeReal::read(reader)?;
        let vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
        let vu_data_block_counter = BCDString::decode(&reader.read_into_vec(2)?)?;

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
