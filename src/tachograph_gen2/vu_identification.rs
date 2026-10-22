use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::VuSoftwareIdentification;
use crate::tacho::{Address, ExtendedSerialNumber, Name, TimeReal, VUTransferResponseParameterID};
use crate::{CodePage, Readable, ReadableWithParams, Result, bytes_to_ia5_fix_string, bytes_to_string};

const VU_PART_NUMBER_LENGTH: u32 = 16;
const VU_APPROVAL_NUMBER_LENGTH: u32 = 16;
const VU_DIGITAL_MAP_VERSION_LENGTH: usize = 12;

/// Information, stored in a vehicle unit, related to the identification of the
/// vehicle unit (Annex 1B requirement 075 and Annex 1C requirement 93 and 121).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2VuIdentification"))]
pub struct VuIdentification {
    #[serde(rename = "isGen2V2")]
    pub is_gen2_v2: bool,
    #[serde(rename = "vuManufacturerName")]
    pub vu_manufacturer_name: Name,
    #[serde(rename = "vuManufacturerAddress")]
    pub vu_manufacturer_address: Address,
    #[serde(rename = "vuPartNumber")]
    pub vu_part_number: String,
    #[serde(rename = "vuSerialNumber")]
    pub vu_serial_number: ExtendedSerialNumber,
    #[serde(rename = "vuSoftwareIdentification")]
    pub vu_software_identification: VuSoftwareIdentification,
    #[serde(rename = "vuManufacturingDate")]
    pub vu_manufacturing_date: TimeReal,
    #[serde(rename = "vuApprovalNumber")]
    pub vu_approval_number: String,
    #[serde(rename = "vuGeneration")]
    pub vu_generation: u8,
    #[serde(rename = "vuAbility")]
    pub vu_ability: u8,
    #[serde(rename = "vuDigitalMapVersion")]
    pub vu_digital_map_version: Option<String>,
}

impl VuIdentification {
    pub(crate) const RECORD_SIZE_V1: u16 = 126;
    pub(crate) const RECORD_SIZE_V2: u16 = 138;
}

impl ReadableWithParams for VuIdentification {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuIdentification> {
        let vu_manufacturer_name = Name::read(reader)?;
        let vu_manufacturer_address = Address::read(reader)?;
        let vu_part_number = bytes_to_string(&reader.read_into_vec(VU_PART_NUMBER_LENGTH)?, &CodePage::IsoIec8859_1); // Code Page 1
        let vu_serial_number = ExtendedSerialNumber::read(reader)?;
        let vu_software_identification = VuSoftwareIdentification::read(reader)?;
        let vu_manufacturing_date = TimeReal::read(reader)?;
        let vu_approval_number = bytes_to_string(&reader.read_into_vec(VU_APPROVAL_NUMBER_LENGTH)?, &CodePage::IsoIec8859_1); // Code Page 1
        let vu_generation = reader.read_u8()?;
        let vu_ability = reader.read_u8()?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2TechnicalData;
        let vu_digital_map_version = if is_gen2_v2 {
            let bytes = reader.read_bytes::<VU_DIGITAL_MAP_VERSION_LENGTH>()?;
            if bytes == [0xFF; VU_DIGITAL_MAP_VERSION_LENGTH] { None } else { Some(bytes_to_ia5_fix_string(&bytes)?) }
        } else {
            None
        };

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
            vu_digital_map_version,
        })
    }
}
