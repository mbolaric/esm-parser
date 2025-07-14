use binary_data::{BinSeek, ReadBytes};

use crate::{Readable, Result, gen2::GeoCoordinate, tacho::TimeReal};

#[derive(Debug)]
pub struct GnssPlaceRecord {
    pub time_stamp: TimeReal,
    pub gnss_accuracy: u8,
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
