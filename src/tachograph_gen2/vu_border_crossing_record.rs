use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::{FullCardNumberAndGeneration, GnssPlaceAuthRecord};
use crate::tacho::{NationNumeric, OdometerShort};
use crate::{Readable, Result};

/// A border crossing stored in a generation 2 version 2 vehicle unit
/// (Annex IC requirements 133a and 133b; Appendix 1, section 2.203a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuBorderCrossingRecord"))]
pub struct VuBorderCrossingRecord {
    #[serde(rename = "cardNumberAndGenDriverSlot")]
    pub card_number_and_gen_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenCodriverSlot")]
    pub card_number_and_gen_co_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "countryLeft")]
    pub country_left: NationNumeric,
    #[serde(rename = "countryEntered")]
    pub country_entered: NationNumeric,
    #[serde(rename = "gnssPlaceAuthRecord")]
    pub gnss_place_auth_record: GnssPlaceAuthRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl VuBorderCrossingRecord {
    pub(crate) const RECORD_SIZE: u16 = 55;
}

impl Readable for VuBorderCrossingRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuBorderCrossingRecord> {
        let card_number_and_gen_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let card_number_and_gen_co_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let country_left: NationNumeric = reader.read_u8()?.into();
        let country_entered: NationNumeric = reader.read_u8()?.into();
        let gnss_place_auth_record = GnssPlaceAuthRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self {
            card_number_and_gen_driver_slot,
            card_number_and_gen_co_driver_slot,
            country_left,
            country_entered,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}
