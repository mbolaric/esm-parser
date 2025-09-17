use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result};

#[derive(Debug)]
pub struct WorkshopCardCalibrationDataParams {
    pub no_of_calibration_records: u8,
}

impl WorkshopCardCalibrationDataParams {
    pub fn new(no_of_calibration_records: u8) -> Self {
        Self { no_of_calibration_records }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkshopCardCalibrationData<T> {
    pub calibration_total_number: u16,
    pub calibration_pointer_newest_record: u8,
    pub calibration_records: Vec<T>,
}

impl<T: Readable<T>> ReadableWithParams<WorkshopCardCalibrationData<T>> for WorkshopCardCalibrationData<T> {
    type P = WorkshopCardCalibrationDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<WorkshopCardCalibrationData<T>> {
        let calibration_total_number = reader.read_u16::<BigEndian>()?;
        let calibration_pointer_newest_record = reader.read_u8()?;
        let mut calibration_records: Vec<T> = Vec::new();
        for _ in 0..params.no_of_calibration_records {
            let workshop_card_calibration_record = T::read(reader)?;
            calibration_records.push(workshop_card_calibration_record);
        }
        Ok(Self { calibration_total_number, calibration_pointer_newest_record, calibration_records })
    }
}
