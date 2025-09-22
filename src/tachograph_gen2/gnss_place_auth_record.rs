use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result, gen2::GeoCoordinate, tacho::TimeReal};

/// Information related to the GNSS position of the vehicle (Annex IC
/// requirements 108, 109, 110, 296, 306a, 306c, 306e, 306g, 356a, 356c, 356e and 356g).
#[derive(Debug, Serialize)]
pub struct GnssPlaceAuthRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "gnssAccuracy")]
    pub gnss_accuracy: u8,
    #[serde(rename = "geoCoordinates")]
    pub geo_coordinates: GeoCoordinate,
    #[serde(rename = "authenticationStatus")]
    pub authentication_status: u8,
}

impl Readable<GnssPlaceAuthRecord> for GnssPlaceAuthRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<GnssPlaceAuthRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let gnss_accuracy = reader.read_u8()?;
        let geo_coordinates = GeoCoordinate::read(reader)?;
        let authentication_status = reader.read_u8()?;

        Ok(Self { time_stamp, gnss_accuracy, geo_coordinates, authentication_status })
    }
}
