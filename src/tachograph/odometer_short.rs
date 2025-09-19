use binary_data::BigEndian;
use serde::Serialize;

use crate::Readable;

/// the odometer value.
#[derive(Debug, Serialize)]
pub struct OdometerShort {
    pub data: Option<u32>,
}

impl Readable<OdometerShort> for OdometerShort {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<OdometerShort> {
        let odometar = reader.read_u24::<BigEndian>()?;
        if odometar == 0xFFFFFF {
            return Ok(Self { data: None });
        }

        Ok(Self { data: Some(odometar) })
    }
}
