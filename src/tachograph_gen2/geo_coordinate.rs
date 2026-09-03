use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, Result};

/// The geo-coordinates are encoded as integers. These integers are multiples
/// of the ±DDMM.M encoding for the latitude and ±DDDMM.M for the
/// longitude. Here ±DD respectively ±DDD denotes the degrees and
/// MM.M the minutes. Longitude and latitude of an unknown position
/// shall be represented as Hex ‘7FFFFF’ (Decimal 8388607).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2GeoCoordinate"))]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoCoordinate {
    fn bytes_to_coordinate(coord_bytes: [u8; 3]) -> f64 {
        let raw = ((coord_bytes[0] as u32) << 16) | ((coord_bytes[1] as u32) << 8) | (coord_bytes[2] as u32);

        let sign = if (raw & 0x800000) != 0 { -1.0 } else { 1.0 }; // Check the 24th bit
        let value = raw & 0x7FFFFF; // Mask out the sign bit (keep only 23 bits)

        // Hex '7FFFFF' (all 23 magnitude bits set) marks an unknown position - not a real DDMM.M value,
        // so it must not be decoded as one.
        if value == 0x7FFFFF {
            return f64::NAN;
        }

        let ddmm_tenths = value as f64 / 10.0; // = DDMM.M (or DDDMM.M)
        let degrees = (ddmm_tenths / 100.0).floor();
        let minutes = ddmm_tenths - degrees * 100.0;
        let decimal_degrees = degrees + minutes / 60.0;

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

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;

    use super::*;

    #[test]
    fn decodes_a_positive_ddmm_tenths_latitude_and_longitude() {
        // Latitude magnitude 47508 (0x00B994) = DDMM.M 4750.8 -> 47 deg,
        // 50.8 min -> 47.846666... N. Longitude magnitude 3334 (0x000D06) = DDMM.M 333.4 -> 3 deg, 33.4 min -> 3.556666... E.
        let mut reader = BinMemoryBuffer::from(vec![0x00, 0xB9, 0x94, 0x00, 0x0D, 0x06]);
        let value = GeoCoordinate::read(&mut reader).expect("geo coordinate should parse");

        assert!((value.latitude - 47.846_666_666_666_67).abs() < 1e-9);
        assert!((value.longitude - 3.556_666_666_666_67).abs() < 1e-9);
    }

    #[test]
    fn decodes_a_negative_latitude_using_the_24th_bit_as_sign() {
        // Same magnitude as above (47508 / 0x00B994) with the sign bit set.
        let mut reader = BinMemoryBuffer::from(vec![0x80, 0xB9, 0x94, 0x00, 0x0D, 0x06]);
        let value = GeoCoordinate::read(&mut reader).expect("geo coordinate should parse");

        assert!((value.latitude - -47.846_666_666_666_67).abs() < 1e-9);
    }

    #[test]
    fn decodes_the_unknown_position_sentinel_as_nan() {
        // Hex '7FFFFF' (all 23 magnitude bits set) is the spec's explicit
        // "unknown position" marker, not a real DDMM.M value.
        let mut reader = BinMemoryBuffer::from(vec![0x7F, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF]);
        let value = GeoCoordinate::read(&mut reader).expect("geo coordinate should parse");

        assert!(value.latitude.is_nan());
        assert!(value.longitude.is_nan());
    }
}
