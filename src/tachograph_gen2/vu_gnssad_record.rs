use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{FullCardNumberAndGeneration, GnssPlaceRecord},
    tacho::{OdometerShort, TimeReal, VUTransferResponseParameterID},
};

#[derive(Debug)]
pub struct VuGnssadRecord {
    pub is_gen2_v2: bool,
    pub time_stamp: TimeReal,
    pub driver_slot_card_number_and_gen: FullCardNumberAndGeneration,
    pub co_driver_slot_card_number_and_gen: FullCardNumberAndGeneration,
    pub gnss_place_record: GnssPlaceRecord,
    pub vehicle_odometer_value: OdometerShort,
}

impl ReadableWithParams<VuGnssadRecord> for VuGnssadRecord {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuGnssadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let driver_slot_card_number_and_gen = FullCardNumberAndGeneration::read(reader)?;
        let co_driver_slot_card_number_and_gen = FullCardNumberAndGeneration::read(reader)?;
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        if is_gen2_v2 {
            let _ = reader.read_u8()?;
        }

        Ok(Self {
            is_gen2_v2,
            time_stamp,
            driver_slot_card_number_and_gen,
            co_driver_slot_card_number_and_gen,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}
