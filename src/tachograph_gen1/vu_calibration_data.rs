use binary_data::BigEndian;
use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{Address, CalibrationPurpose, FullCardNumber, Name, OdometerShort, TimeReal, VehicleRegistrationIdentification},
};

/// Information, stored in a vehicle unit, related a calibration of the
/// recording equipment (Annex 1B requirement 098 and Annex 1C requirement 119 and 120).
#[derive(Debug, Serialize)]
pub struct VuCalibrationRecord {
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
}

impl Readable<VuCalibrationRecord> for VuCalibrationRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuCalibrationRecord> {
        let calibration_purpose = reader.read_u8()?.into();
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number = FullCardNumber::read(reader)?;
        let workshop_card_expiry_date = TimeReal::read(reader)?;
        let vehicle_identification_number = bytes_to_ia5_fix_string(&reader.read_into_vec(17)?)?;
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let w_vehicle_characteristic_constant: u16 = reader.read_u16::<BigEndian>()?;
        let k_constant_of_recording_equipment: u16 = reader.read_u16::<BigEndian>()?;
        let l_tyre_circumference: u16 = reader.read_u16::<BigEndian>()?;
        let tyre_size = bytes_to_ia5_fix_string(&reader.read_into_vec(15)?)?;
        let authorised_speed = reader.read_u8()?;
        let old_odometer_value = OdometerShort::read(reader)?;
        let new_odometer_value = OdometerShort::read(reader)?;
        let old_time_value = TimeReal::read(reader)?;
        let new_time_value = TimeReal::read(reader)?;
        let next_calibration_date = TimeReal::read(reader)?;

        Ok(Self {
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
        })
    }
}

#[derive(Debug, Serialize)]
pub struct VUCalibrationData {
    pub no_of_vu_calibrations: u8,
    pub calibrations: Vec<VuCalibrationRecord>,
}

impl Readable<VUCalibrationData> for VUCalibrationData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VUCalibrationData> {
        let no_of_vu_calibrations = reader.read_u8()?;
        let mut vu_calibrations: Vec<VuCalibrationRecord> = Vec::new();
        for _ in 0..no_of_vu_calibrations {
            vu_calibrations.push(VuCalibrationRecord::read(reader)?);
        }

        Ok(Self { no_of_vu_calibrations, calibrations: vu_calibrations })
    }
}
