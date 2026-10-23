use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::{FullCardNumberAndGeneration, GnssPlaceAuthRecord, GnssPlaceRecord};
use crate::tacho::{OdometerShort, RecordType, TimeReal, VUTransferResponseParameterID, VuRecordLayout};
use crate::{Readable, ReadableWithParams, Result};

/// Information, stored in a vehicle unit, related to the GNSS position of the
/// vehicle if the accumulated driving time reaches a multiple of three hours
/// (Annex IC requirement 108, 110).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuGnssadRecord"))]
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
    pub gnss_place_record: Option<GnssPlaceRecord>,
    #[serde(rename = "gnssPlaceAuthRecord")]
    pub gnss_place_auth_record: Option<GnssPlaceAuthRecord>,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl VuGnssadRecord {
    pub(crate) const RECORD_SIZE_V1: u16 = 56;
    pub(crate) const RECORD_SIZE_V2: u16 = 57;
}

impl ReadableWithParams for VuGnssadRecord {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuGnssadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let card_number_and_gen_co_driver_slot = FullCardNumberAndGeneration::read(reader)?;
        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        let gnss_place_record = if !is_gen2_v2 { Some(GnssPlaceRecord::read(reader)?) } else { None };
        let gnss_place_auth_record = if is_gen2_v2 { Some(GnssPlaceAuthRecord::read(reader)?) } else { None };
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self {
            is_gen2_v2,
            time_stamp,
            card_number_and_gen_driver_slot,
            card_number_and_gen_co_driver_slot,
            gnss_place_record,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

impl VuRecordLayout for VuGnssadRecord {
    const RECORD_TYPE: RecordType = RecordType::VuGNSSADRecord;
    const RECORD_SIZE: u16 = 56;

    fn record_size(trep_id: VUTransferResponseParameterID) -> u16 {
        if trep_id == VUTransferResponseParameterID::Gen2v2Activities { Self::RECORD_SIZE_V2 } else { Self::RECORD_SIZE_V1 }
    }
}
