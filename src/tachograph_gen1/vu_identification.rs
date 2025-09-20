use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    gen1::SoftwareIdentification,
    tacho::{Address, ExtendedSerialNumber, Name, TimeReal},
};

/// Information, stored in a vehicle unit, related to the identification of the
/// vehicle unit (Annex 1B requirement 075 and Annex 1C requirement 93 and 121).
#[derive(Debug, Serialize)]
pub struct VUIdentification {
    #[serde(rename = "vuManufacturerName")]
    pub vu_manufacturer_name: Name,
    #[serde(rename = "vuManufacturerAddress")]
    pub vu_manufacturer_address: Address,
    #[serde(rename = "vuPartNumber")]
    pub vu_part_number: String,
    #[serde(rename = "vuSerialNumber")]
    pub vu_serial_number: ExtendedSerialNumber,
    #[serde(rename = "vuSoftwareIdentification")]
    pub vu_software_identification: SoftwareIdentification,
    #[serde(rename = "vuManufacturingDate")]
    pub vu_manufacturing_date: TimeReal,
    #[serde(rename = "vuApprovalNumber")]
    pub vu_approval_number: String,
}

impl Readable<VUIdentification> for VUIdentification {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VUIdentification> {
        let vu_manufacturer_name = Name::read(reader)?;
        let vu_manufacturer_address = Address::read(reader)?;
        let vu_part_number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?.trim().to_string();
        let vu_serial_number = ExtendedSerialNumber::read(reader)?;
        let vu_software_identification = SoftwareIdentification::read(reader)?;

        let vu_manufacturing_date = TimeReal::read(reader)?;
        let vu_approval_number = bytes_to_ia5_fix_string(&reader.read_into_vec(8)?)?.trim().to_string();

        Ok(Self {
            vu_manufacturer_name,
            vu_manufacturer_address,
            vu_part_number,
            vu_serial_number,
            vu_software_identification,
            vu_manufacturing_date,
            vu_approval_number,
        })
    }
}
