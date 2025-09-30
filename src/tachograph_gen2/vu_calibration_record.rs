use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result, bytes_to_ia5_fix_string,
    gen2::SealDataVu,
    tacho::{
        Address, CalibrationPurpose, FullCardNumber, Name, OdometerShort, TimeReal, VUTransferResponseParameterID,
        VehicleRegistrationIdentification,
    },
};

/// Information, stored in a vehicle unit, related a calibration of the
/// recording equipment (Annex 1B requirement 098 and Annex 1C requirement 119 and 120).
#[derive(Debug, Serialize)]
pub struct VuCalibrationRecord {
    #[serde(rename = "isGen2V2")]
    pub is_gen2_v2: bool,
    #[serde(rename = "calibrationPurpose")]
    pub calibration_purpose: CalibrationPurpose,
    #[serde(rename = "workshopName")]
    pub workshop_name: Name,
    #[serde(rename = "workshopAddress")]
    pub workshop_address: Address,
    #[serde(rename = "workshopCardNumber")]
    pub workshop_card_number: FullCardNumber,
    #[serde(rename = "workshopCardExpiryDate")]
    pub workshop_card_expiry_date: TimeReal,
    #[serde(rename = "vehicleIdentificationNumber")]
    pub vehicle_identification_number: String,
    #[serde(rename = "vehicleRegistrationIdentification")]
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    #[serde(rename = "wVehicleCharacteristicConstant")]
    pub w_vehicle_characteristic_constant: u16,
    #[serde(rename = "kConstantOfRecordingEquipment")]
    pub k_constant_of_recording_equipment: u16,
    #[serde(rename = "lTyreCircumference")]
    pub l_tyre_circumference: u16,
    #[serde(rename = "tyreSize")]
    pub tyre_size: String,
    #[serde(rename = "authorisedSpeed")]
    pub authorised_speed: u8,
    #[serde(rename = "oldOdometerValue")]
    pub old_odometer_value: OdometerShort,
    #[serde(rename = "newOdometerValue")]
    pub new_odometer_value: OdometerShort,
    #[serde(rename = "oldTimeValue")]
    pub old_time_value: TimeReal,
    #[serde(rename = "newTimeValue")]
    pub new_time_value: TimeReal,
    #[serde(rename = "nextCalibrationDate")]
    pub next_calibration_date: TimeReal,
    #[serde(rename = "sealDataVu")]
    pub seal_data_vu: SealDataVu,
}

const VEHICLE_IDENTIFICATION_NUMBER_LENGTH: u32 = 17;
const TYRE_SIZE_LENGTH: u32 = 15;

impl ReadableWithParams<VuCalibrationRecord> for VuCalibrationRecord {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuCalibrationRecord> {
        let calibration_purpose: CalibrationPurpose = reader.read_u8()?.into();
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number = FullCardNumber::read(reader)?;
        let workshop_card_expiry_date = TimeReal::read(reader)?;
        let vehicle_identification_number =
            bytes_to_ia5_fix_string(&reader.read_into_vec(VEHICLE_IDENTIFICATION_NUMBER_LENGTH)?)?;
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let w_vehicle_characteristic_constant = reader.read_u16::<BigEndian>()?;
        let k_constant_of_recording_equipment = reader.read_u16::<BigEndian>()?;
        let l_tyre_circumference = reader.read_u16::<BigEndian>()?;
        let tyre_size = bytes_to_ia5_fix_string(&reader.read_into_vec(TYRE_SIZE_LENGTH)?)?;
        let authorised_speed = reader.read_u8()?;
        let old_odometer_value = OdometerShort::read(reader)?;
        let new_odometer_value = OdometerShort::read(reader)?;
        let old_time_value = TimeReal::read(reader)?;
        let new_time_value = TimeReal::read(reader)?;
        let next_calibration_date = TimeReal::read(reader)?;
        let seal_data_vu = SealDataVu::read(reader)?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        if is_gen2_v2 {
            // TODO: not implemented for now.
            let _ = reader.read_bytes::<30>()?;
        }

        Ok(Self {
            is_gen2_v2,
            calibration_purpose,
            workshop_name,
            workshop_address,
            workshop_card_number,
            workshop_card_expiry_date,
            vehicle_identification_number,
            vehicle_registration_identification,
            w_vehicle_characteristic_constant,
            k_constant_of_recording_equipment,
            l_tyre_circumference,
            tyre_size,
            authorised_speed,
            old_odometer_value,
            new_odometer_value,
            old_time_value,
            new_time_value,
            next_calibration_date,
            seal_data_vu,
        })
    }
}
