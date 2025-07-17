use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{
    Readable, ReadableWithParams, Result, bytes_to_ia5_fix_string,
    tacho::{CalibrationPurpose, ExtendedSerialNumber, OdometerShort, TimeReal, VehicleRegistrationIdentification},
};

#[derive(Debug)]
pub struct WorkshopCardCalibrationDataParams {
    pub no_of_calibration_records: u8,
}

impl WorkshopCardCalibrationDataParams {
    pub fn new(no_of_calibration_records: u8) -> Self {
        Self { no_of_calibration_records }
    }
}

#[derive(Debug)]
pub struct WorkshopCardCalibrationRecord {
    pub calibration_purpose: CalibrationPurpose,
    pub vehicle_identification_number: String,
    pub vehicle_registration: VehicleRegistrationIdentification,
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
    pub vu_part_number: String,
    pub vu_serial_number: ExtendedSerialNumber,
    pub sensor_serial_number: ExtendedSerialNumber,
}

impl Readable<WorkshopCardCalibrationRecord> for WorkshopCardCalibrationRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<WorkshopCardCalibrationRecord> {
        let calibration_purpose: CalibrationPurpose = reader.read_u8()?.into();
        let vehicle_identification_number = bytes_to_ia5_fix_string(&reader.read_into_vec(17)?)?;
        let vehicle_registration = VehicleRegistrationIdentification::read(reader)?;
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
        let vu_part_number = bytes_to_ia5_fix_string(&reader.read_into_vec(16)?)?;
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

#[derive(Debug)]
pub struct WorkshopCardCalibrationData {
    pub calibration_total_number: u16,
    pub calibration_pointer_newest_record: u8,
    pub calibration_records: Vec<WorkshopCardCalibrationRecord>,
}

impl ReadableWithParams<WorkshopCardCalibrationData> for WorkshopCardCalibrationData {
    type P = WorkshopCardCalibrationDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<WorkshopCardCalibrationData> {
        let calibration_total_number = reader.read_u16::<BigEndian>()?;
        let calibration_pointer_newest_record = reader.read_u8()?;
        let mut calibration_records: Vec<WorkshopCardCalibrationRecord> = Vec::new();
        for _ in 0..params.no_of_calibration_records {
            let workshop_card_calibration_record = WorkshopCardCalibrationRecord::read(reader)?;
            calibration_records.push(workshop_card_calibration_record);
        }
        Ok(Self { calibration_total_number, calibration_pointer_newest_record, calibration_records })
    }
}
