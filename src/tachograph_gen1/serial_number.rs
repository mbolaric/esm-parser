use binary_data::BigEndian;

use crate::{BCDString, Readable};

#[derive(Debug)]

pub struct SerialNumber {
    pub serial_number: u32,
    pub month_year: String,
    pub serial_type: u8,
    pub manufacturer_code: u8,
}

impl Readable<SerialNumber> for SerialNumber {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<SerialNumber> {
        let serial_number = reader.read_u32::<BigEndian>()?;
        let month_year = BCDString::decode(&reader.read_bytes::<2>()?);
        let serial_type = reader.read_u8()?;
        let manufacturer_code = reader.read_u8()?;

        Ok(Self { serial_number, month_year, serial_type, manufacturer_code })
    }
}
