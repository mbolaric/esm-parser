use binary_data::BigEndian;

use crate::{
    gen1::{Address, FullCardNumber, Name, VehicleRegistrationIdentification},
    helpers::vec_u8_to_string,
    tacho::{CalibrationPurposeCode, OdometerShort, TimeReal},
    Readable,
};

#[derive(Debug)]
pub struct VuCalibrationRecord {
    pub calibration_purpose: CalibrationPurposeCode,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumber,
    pub workshop_card_expiry_date: TimeReal,
    pub vehicle_identification_number: String,
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub w_vehicle_characteristic_constant: u16,
    pub k_constant_of_recording_equipment: u16,
    pub l_tyre_circumference: u16,
    pub tyre_size: String,
    pub authorised_speed: u8,
    pub old_odometer_value: OdometerShort,
    pub new_odometer_value: OdometerShort,
    pub old_time_value: TimeReal,
    pub new_time_value: TimeReal,
    pub next_calibration_date: TimeReal,
}

impl Readable<VuCalibrationRecord> for VuCalibrationRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VuCalibrationRecord> {
        let calibration_purpose = reader.read_u8()?.into();
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number = FullCardNumber::read(reader)?;
        let workshop_card_expiry_date = TimeReal::read(reader)?;
        let vehicle_identification_number = vec_u8_to_string(reader.read_into_vec(17)?)?;
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let w_vehicle_characteristic_constant: u16 = reader.read_u16::<BigEndian>()?;
        let k_constant_of_recording_equipment: u16 = reader.read_u16::<BigEndian>()?;
        let l_tyre_circumference: u16 = reader.read_u16::<BigEndian>()?;
        let tyre_size = vec_u8_to_string(reader.read_into_vec(15)?)?;
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

#[derive(Debug)]
pub struct VUCalibrationData {
    pub no_of_vu_calibrations: u8,
    pub calibrations: Vec<VuCalibrationRecord>,
}

impl Readable<VUCalibrationData> for VUCalibrationData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(
        reader: &mut R,
    ) -> crate::Result<VUCalibrationData> {
        let no_of_vu_calibrations = reader.read_u8()?;
        let mut vu_calibrations: Vec<VuCalibrationRecord> = Vec::new();
        for _ in 0..no_of_vu_calibrations {
            vu_calibrations.push(VuCalibrationRecord::read(reader)?);
        }

        Ok(Self {
            no_of_vu_calibrations,
            calibrations: vu_calibrations,
        })
    }
}
