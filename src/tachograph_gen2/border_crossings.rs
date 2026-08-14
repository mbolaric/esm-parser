use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::GnssPlaceRecord;
use crate::tacho::{NationNumeric, OdometerShort, TimeReal};
use crate::{Readable, ReadableWithParams, Result};

/// Information, stored in a driver or workshop card, related to border crossings
/// (Annex IC requirements 306h and 356h)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2BorderCrossingRecord"))]
pub struct BorderCrossingRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "countryLeft")]
    pub country_left: NationNumeric,
    #[serde(rename = "countryEntered")]
    pub country_entered: NationNumeric,
    #[serde(rename = "gnssPlaceRecord")]
    pub gnss_place_record: GnssPlaceRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<BorderCrossingRecord> for BorderCrossingRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<BorderCrossingRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let country_left: NationNumeric = reader.read_u8()?.into();
        let country_entered: NationNumeric = reader.read_u8()?.into();
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self { time_stamp, country_left, country_entered, gnss_place_record, vehicle_odometer_value })
    }
}

#[derive(Debug)]
pub struct BorderCrossingsParams {
    pub no_of_border_crossing_records: u16,
}

impl BorderCrossingsParams {
    pub fn new(no_of_border_crossing_records: u16) -> Self {
        Self { no_of_border_crossing_records }
    }
}

/// Information, stored in a driver or workshop card, related to border crossings
/// (Annex IC requirements 306i and 356i)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2BorderCrossings"))]
pub struct BorderCrossings {
    #[serde(rename = "borderCrossingPointerNewestRecord")]
    pub border_crossing_pointer_newest_record: u16,
    #[serde(rename = "cardBorderCrossingRecords")]
    pub card_border_crossing_records: Vec<BorderCrossingRecord>,
}

impl ReadableWithParams<BorderCrossings> for BorderCrossings {
    type P = BorderCrossingsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<BorderCrossings> {
        let border_crossing_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut records: Vec<BorderCrossingRecord> = Vec::new();
        for _ in 0..params.no_of_border_crossing_records {
            let record = BorderCrossingRecord::read(reader)?;
            if record.time_stamp.has_data() {
                records.push(record);
            }
        }

        Ok(Self { border_crossing_pointer_newest_record, card_border_crossing_records: records })
    }
}
