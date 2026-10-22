use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::{FullCardNumberAndGeneration, GnssPlaceAuthRecord, OperationType};
use crate::tacho::{OdometerShort, TimeReal};
use crate::{Readable, Result};

/// A load/unload operation stored in a generation 2 version 2 vehicle unit
/// (Annex IC requirements 133e, 133f and 133g; Appendix 1, section 2.208a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuLoadUnloadRecord"))]
pub struct VuLoadUnloadRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "operationType")]
    pub operation_type: OperationType,
    #[serde(rename = "cardNumberAndGenDriverSlot")]
    pub card_number_and_gen_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "cardNumberAndGenCodriverSlot")]
    pub card_number_and_gen_co_driver_slot: FullCardNumberAndGeneration,
    #[serde(rename = "gnssPlaceAuthRecord")]
    pub gnss_place_auth_record: GnssPlaceAuthRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl VuLoadUnloadRecord {
    pub(crate) const RECORD_SIZE: u16 = 58;
}

impl Readable for VuLoadUnloadRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuLoadUnloadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let operation_type: OperationType = reader.read_u8()?.into();
        let card_number_and_gen_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let card_number_and_gen_co_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let gnss_place_auth_record = GnssPlaceAuthRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self {
            time_stamp,
            operation_type,
            card_number_and_gen_driver_slot,
            card_number_and_gen_co_driver_slot,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}
