use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{
    Readable, ReadableWithParams, Result, bytes_to_ia5_fix_string,
    gen2::SealDataVu,
    tacho::{
        Address, CalibrationPurpose, FullCardNumber, Name, OdometerShort, TimeReal, VUTransferResponseParameterID,
        VehicleRegistrationIdentification,
    },
};

#[derive(Debug)]
pub struct VuCalibrationRecord {
    pub is_gen2_v2: bool,
    pub calibration_purpose: CalibrationPurpose,
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
    pub seal_data_vu: SealDataVu,
}

impl ReadableWithParams<VuCalibrationRecord> for VuCalibrationRecord {
    type P = VUTransferResponseParameterID;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<VuCalibrationRecord> {
        let calibration_purpose: CalibrationPurpose = reader.read_u8()?.into();
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number = FullCardNumber::read(reader)?;
        let workshop_card_expiry_date = TimeReal::read(reader)?;
        let vehicle_identification_number = bytes_to_ia5_fix_string(&reader.read_into_vec(17)?)?;
        let vehicle_registration_identification = VehicleRegistrationIdentification::read(reader)?;
        let w_vehicle_characteristic_constant = reader.read_u16::<BigEndian>()?;
        let k_constant_of_recording_equipment = reader.read_u16::<BigEndian>()?;
        let l_tyre_circumference = reader.read_u16::<BigEndian>()?;
        let tyre_size = bytes_to_ia5_fix_string(&reader.read_into_vec(15)?)?;
        let authorised_speed = reader.read_u8()?;
        let old_odometer_value = OdometerShort::read(reader)?;
        let new_odometer_value = OdometerShort::read(reader)?;
        let old_time_value = TimeReal::read(reader)?;
        let new_time_value = TimeReal::read(reader)?;
        let next_calibration_date = TimeReal::read(reader)?;
        let seal_data_vu = SealDataVu::read(reader)?;

        let is_gen2_v2: bool = *params == VUTransferResponseParameterID::Gen2v2Activities;
        if is_gen2_v2 {
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
