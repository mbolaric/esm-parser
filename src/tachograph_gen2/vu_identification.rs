use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::VuSoftwareIdentification,
    helpers::vec_u8_to_string,
    tacho::{Address, ExtendedSerialNumber, Name, TimeReal},
};

#[derive(Debug)]
pub struct VuIdentification {
    pub vu_manufacturer_name: Name,
    pub vu_manufacturer_address: Address,
    pub vu_part_number: String,
    pub vu_serial_number: ExtendedSerialNumber,
    pub vu_software_identification: VuSoftwareIdentification,
    pub vu_manufacturing_date: TimeReal,
    pub vu_approval_number: String,
    pub vu_generation: u8,
    pub vu_ability: u8,
}

impl Readable<VuIdentification> for VuIdentification {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuIdentification> {
        let vu_manufacturer_name = Name::read(reader)?;
        let vu_manufacturer_address = Address::read(reader)?;
        let vu_part_number = vec_u8_to_string(reader.read_into_vec(16)?)?; // Code Page 1
        let vu_serial_number = ExtendedSerialNumber::read(reader)?;
        let vu_software_identification = VuSoftwareIdentification::read(reader)?;
        let vu_manufacturing_date = TimeReal::read(reader)?;
        let vu_approval_number = vec_u8_to_string(reader.read_into_vec(16)?)?; // Code Page 1
        let vu_generation = reader.read_u8()?;
        let vu_ability = reader.read_u8()?;

        Ok(Self {
            vu_manufacturer_name,
            vu_manufacturer_address,
            vu_part_number,
            vu_serial_number,
            vu_software_identification,
            vu_manufacturing_date,
            vu_approval_number,
            vu_generation,
            vu_ability,
        })
    }
}
