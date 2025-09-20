use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::GeoCoordinate, tacho::TimeReal};

/// Information related to the GNSS position of the vehicle (Annex 1C
/// requirements 108, 109, 110, 296, 305, 347, and 353).
#[derive(Debug, Serialize)]
pub struct GnssPlaceRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "gnssAccuracy")]
    pub gnss_accuracy: u8,
    #[serde(rename = "geoCoordinates")]
    pub geo_coordinates: GeoCoordinate,
}

impl Readable<GnssPlaceRecord> for GnssPlaceRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<GnssPlaceRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let gnss_accuracy = reader.read_u8()?;
        let geo_coordinates = GeoCoordinate::read(reader)?;

        Ok(Self { time_stamp, gnss_accuracy, geo_coordinates })
    }
}
