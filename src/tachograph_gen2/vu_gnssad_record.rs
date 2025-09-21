use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::{FullCardNumberAndGeneration, GnssPlaceRecord},
    tacho::{OdometerShort, TimeReal, VUTransferResponseParameterID},
};

/// Information, stored in a vehicle unit, related to the GNSS position of the
/// vehicle if the accumulated driving time reaches a multiple of three hours
/// (Annex IC requirement 108, 110).
#[derive(Debug, Serialize)]
pub struct VuGnssadRecord {
    #[serde(rename = "isGen2V2")]
    pub is_gen2_v2: bool,
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "cardNumberAndGenDriverSlot")]
    pub card_number_and_gen_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenCodriverSlot")]
    pub card_number_and_gen_co_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "gnssPlaceRecord")]
    pub gnss_place_record: GnssPlaceRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl ReadableWithParams<VuGnssadRecord> for VuGnssadRecord {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuGnssadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let card_number_and_gen_co_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        if is_gen2_v2 {
            let _ = reader.read_u8()?;
        }

        Ok(Self {
            is_gen2_v2,
            time_stamp,
            card_number_and_gen_driver_slot,
            card_number_and_gen_co_driver_slot,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}
