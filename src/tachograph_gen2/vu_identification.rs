use binary_data::{BinSeek, ReadBytes};

use crate::{
    CodePage, Readable, ReadableWithParams, Result, bytes_to_string,
    gen2::VuSoftwareIdentification,
    tacho::{Address, ExtendedSerialNumber, Name, TimeReal, VUTransferResponseParameterID},
};

#[derive(Debug)]
pub struct VuIdentification {
    pub is_gen2_v2: bool,
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

impl ReadableWithParams<VuIdentification> for VuIdentification {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuIdentification> {
        let vu_manufacturer_name = Name::read(reader)?;
        let vu_manufacturer_address = Address::read(reader)?;
        let vu_part_number = bytes_to_string(&reader.read_into_vec(16)?, &CodePage::IsoIec8859_1); // Code Page 1
        let vu_serial_number = ExtendedSerialNumber::read(reader)?;
        let vu_software_identification = VuSoftwareIdentification::read(reader)?;
        let vu_manufacturing_date = TimeReal::read(reader)?;
        let vu_approval_number = bytes_to_string(&reader.read_into_vec(16)?, &CodePage::IsoIec8859_1); // Code Page 1
        let vu_generation = reader.read_u8()?;
        let vu_ability = reader.read_u8()?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        if is_gen2_v2 {
            let _ = reader.read_bytes::<12>()?;
        }

        Ok(Self {
            is_gen2_v2,
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
