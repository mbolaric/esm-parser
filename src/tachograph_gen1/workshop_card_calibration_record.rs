use binary_data::BigEndian;
use serde::Serialize;

use crate::{
    Readable, bytes_to_ia5_fix_string,
    tacho::{CalibrationPurpose, ExtendedSerialNumber, OdometerShort, TimeReal, VehicleRegistrationIdentification},
};

const VEHICLE_IDENTIFICATION_NUMBER_LENGTH: u32 = 17;
const TYRE_SIZE_LENGTH: u32 = 15;
const VU_PART_NUMBER_LENGTH: u32 = 16;

/// Information, stored in a workshop card, related to a calibration
/// performed with the card (Annex 1C requirement 314 and 337).
#[derive(Debug, Serialize)]
pub struct WorkshopCardCalibrationRecord {
    #[serde(rename = "calibrationPurpose")]
    pub calibration_purpose: CalibrationPurpose,
    #[serde(rename = "vehicleIdentificationNumber")]
    pub vehicle_identification_number: String,
    #[serde(rename = "vehicleRegistration")]
    pub vehicle_registration: VehicleRegistrationIdentification,
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
    #[serde(rename = "vuPartNumber")]
    pub vu_part_number: String,
    #[serde(rename = "vuSerialNumber")]
    pub vu_serial_number: ExtendedSerialNumber,
    #[serde(rename = "sensorSerialNumber")]
    pub sensor_serial_number: ExtendedSerialNumber,
}

impl Readable<WorkshopCardCalibrationRecord> for WorkshopCardCalibrationRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<WorkshopCardCalibrationRecord> {
        let calibration_purpose: CalibrationPurpose = reader.read_u8()?.into();
        let vehicle_identification_number =
            bytes_to_ia5_fix_string(&reader.read_into_vec(VEHICLE_IDENTIFICATION_NUMBER_LENGTH)?)?;
        let vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
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
        let vu_part_number = bytes_to_ia5_fix_string(&reader.read_into_vec(VU_PART_NUMBER_LENGTH)?)?;
        let vu_serial_number = ExtendedSerialNumber::read(reader)?;
        let sensor_serial_number = ExtendedSerialNumber::read(reader)?;

        Ok(Self {
            calibration_purpose,
            vehicle_identification_number,
            vehicle_registration,
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
            vu_part_number,
            vu_serial_number,
            sensor_serial_number,
        })
    }
}
