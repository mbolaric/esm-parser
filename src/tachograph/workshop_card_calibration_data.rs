use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, read_records};

#[derive(Debug)]
pub struct WorkshopCardCalibrationDataParams {
    pub no_of_calibration_records: u8,
}

impl WorkshopCardCalibrationDataParams {
    pub fn new(no_of_calibration_records: u8) -> Self {
        Self { no_of_calibration_records }
    }
}

/// Information, stored in a workshop card, related to workshop activity
/// performed with the card (Annex 1C requirements 314, 316, 337, and 339).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct WorkshopCardCalibrationData<T> {
    #[serde(rename = "calibrationTotalNumber")]
    pub calibration_total_number: u16,
    #[serde(rename = "calibrationPointerNewestRecord")]
    pub calibration_pointer_newest_record: u8,
    #[serde(rename = "calibrationRecords")]
    pub calibration_records: Vec<T>,
}

impl<T: Readable> ReadableWithParams for WorkshopCardCalibrationData<T> {
    type P = WorkshopCardCalibrationDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<WorkshopCardCalibrationData<T>> {
        let calibration_total_number = reader.read_u16::<BigEndian>()?;
        let calibration_pointer_newest_record = reader.read_u8()?;
        let calibration_records = read_records(reader, params.no_of_calibration_records as usize, |reader| T::read(reader))?;
        Ok(Self { calibration_total_number, calibration_pointer_newest_record, calibration_records })
    }
}
