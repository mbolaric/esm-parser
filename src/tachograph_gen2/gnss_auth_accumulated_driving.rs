use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::CardAuthenticationRecordsParams;
use crate::tacho::TimeReal;
use crate::tachograph_gen2::card_records::read_card_records;
use crate::{Readable, ReadableWithParams, Result};

/// Authentication of an accumulated driving position, matched by timeStamp (Appendix 1, 2.79b).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2GnssAuthStatusADRecord"))]
pub struct GnssAuthStatusADRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "authenticationStatus")]
    pub authentication_status: u8,
}

impl Readable for GnssAuthStatusADRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<Self> {
        let time_stamp = TimeReal::read(reader)?;
        let authentication_status = reader.read_u8()?;
        Ok(Self { time_stamp, authentication_status })
    }
}

/// Gen2 V2 EF GNSS_Places_Authentication, retaining every physical slot (Appendix 1, 2.79a).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2GnssAuthAccumulatedDriving"))]
pub struct GnssAuthAccumulatedDriving {
    #[serde(rename = "gnssAuthADPointerNewestRecord")]
    pub gnss_auth_ad_pointer_newest_record: u16,
    #[serde(rename = "gnssAuthStatusADRecords")]
    pub gnss_auth_status_ad_records: Vec<GnssAuthStatusADRecord>,
}

impl ReadableWithParams for GnssAuthAccumulatedDriving {
    type P = CardAuthenticationRecordsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Self> {
        let (gnss_auth_ad_pointer_newest_record, gnss_auth_status_ad_records) =
            read_card_records(reader, params.no_of_records, 5, "Card authentication")?;
        Ok(Self { gnss_auth_ad_pointer_newest_record, gnss_auth_status_ad_records })
    }
}
