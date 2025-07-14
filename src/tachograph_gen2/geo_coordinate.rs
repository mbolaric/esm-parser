use binary_data::{BinSeek, ReadBytes};

use crate::{Readable, Result};

#[derive(Debug)]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoCoordinate {
    fn bytes_to_coordinate(coord_bytes: [u8; 3]) -> f64 {
        let raw = ((coord_bytes[0] as u32) << 16) | ((coord_bytes[1] as u32) << 8) | (coord_bytes[2] as u32);

        let sign = if (raw & 0x800000) != 0 { -1.0 } else { 1.0 }; // Check the 24th bit
        let value = raw & 0x7FFFFF; // Mask out the sign bit (keep only 23 bits)

        let minutes = value as f64 / 10000.0;
        let decimal_degrees = minutes / 60.0;

        sign * decimal_degrees
    }
}

impl Readable<GeoCoordinate> for GeoCoordinate {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<GeoCoordinate> {
        let latitude_bytes = reader.read_bytes::<3>()?;
        let longitude_bytes = reader.read_bytes::<3>()?;
        let latitude = GeoCoordinate::bytes_to_coordinate(latitude_bytes);
        let longitude = GeoCoordinate::bytes_to_coordinate(longitude_bytes);

        Ok(Self { latitude, longitude })
    }
}
