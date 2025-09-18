use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, tacho::TimeReal};

pub trait VehicleUse {
    fn get_vehicle_first_use(&self) -> &TimeReal;
    fn get_vehicle_last_use(&self) -> &TimeReal;
}

#[derive(Debug)]
pub struct VehiclesUsedParams {
    pub no_of_card_vehicle_records: u32,
}

impl VehiclesUsedParams {
    pub fn new(no_of_card_vehicle_records: u32) -> Self {
        Self { no_of_card_vehicle_records }
    }
}

/// Information, stored in a driver or workshop card, related to the vehicles
/// used by the card holder (Annex 1C requirements 270, 295, 323, and 346).
#[derive(Debug, Serialize)]
pub struct CardVehiclesUsed<T> {
    #[serde(rename = "vehiclePointerNewestRecord")]
    pub vehicle_pointer_newest_record: u16,
    #[serde(rename = "cardVehicleRecords")]
    pub vehicle_records: Vec<T>,
}

impl<T: Readable<T> + VehicleUse> ReadableWithParams<CardVehiclesUsed<T>> for CardVehiclesUsed<T> {
    type P = VehiclesUsedParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardVehiclesUsed<T>> {
        let vehicle_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut vehicle_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_card_vehicle_records {
            let record = T::read(reader)?;
            if record.get_vehicle_first_use().has_data() || record.get_vehicle_last_use().has_data() {
                vehicle_records.push(record);
            }
        }

        Ok(Self { vehicle_pointer_newest_record, vehicle_records })
    }
}
