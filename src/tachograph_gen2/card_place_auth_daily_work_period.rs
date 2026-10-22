use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::CardAuthenticationRecordsParams;
use crate::tacho::TimeReal;
use crate::tachograph_gen2::card_authentication::read_card_authentication_records;
use crate::{Readable, ReadableWithParams, Result};

/// Authentication of a daily work period place, matched to PlaceRecord by entryTime (Appendix 1, 2.116b).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2PlaceAuthStatusRecord"))]
pub struct PlaceAuthStatusRecord {
    #[serde(rename = "entryTime")]
    pub entry_time: TimeReal,
    #[serde(rename = "authenticationStatus")]
    pub authentication_status: u8,
}

impl Readable for PlaceAuthStatusRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
        let entry_time = TimeReal::read(reader)?;
        let authentication_status = reader.read_u8()?;
        Ok(Self { entry_time, authentication_status })
    }
}

/// Gen2 V2 EF Places_Authentication, retaining every physical slot (Appendix 1, 2.26a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2CardPlaceAuthDailyWorkPeriod"))]
pub struct CardPlaceAuthDailyWorkPeriod {
    #[serde(rename = "placeAuthPointerNewestRecord")]
    pub place_auth_pointer_newest_record: u16,
    #[serde(rename = "placeAuthStatusRecords")]
    pub place_auth_status_records: Vec<PlaceAuthStatusRecord>,
}

impl ReadableWithParams for CardPlaceAuthDailyWorkPeriod {
    type P = CardAuthenticationRecordsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Self> {
        let (place_auth_pointer_newest_record, place_auth_status_records) = read_card_authentication_records(reader, params)?;
        Ok(Self { place_auth_pointer_newest_record, place_auth_status_records })
    }
}
