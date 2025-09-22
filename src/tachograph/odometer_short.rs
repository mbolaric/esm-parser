use binary_data::BigEndian;
use serde::Serialize;

use crate::Readable;

/// the odometer value.
#[derive(Debug)]
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

impl Serialize for OdometerShort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(val) = self.data { serializer.serialize_u32(val) } else { serializer.serialize_none() }
    }
}
