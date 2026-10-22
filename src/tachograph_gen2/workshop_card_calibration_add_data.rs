use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::WorkshopCardCalibrationAddDataRecord;
use crate::tachograph_gen2::card_records::read_card_records;
use crate::{Error, ReadableWithParams, Result};

#[derive(Debug)]
pub struct WorkshopCardCalibrationAddDataParams {
    pub no_of_calibration_records: u16,
}

impl WorkshopCardCalibrationAddDataParams {
    pub fn new(no_of_calibration_records: u16) -> Self {
        Self { no_of_calibration_records }
    }
}

/// Additional calibration data with its own physical slots and pointer (Appendix 1, section 2.234b).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2WorkshopCardCalibrationAddData"))]
pub struct WorkshopCardCalibrationAddData {
    #[serde(rename = "calibrationPointerNewestRecord")]
    pub calibration_pointer_newest_record: u16,
    #[serde(rename = "workshopCardCalibrationAddDataRecords")]
    pub workshop_card_calibration_add_data_records: Vec<WorkshopCardCalibrationAddDataRecord>,
}

impl ReadableWithParams for WorkshopCardCalibrationAddData {
    type P = WorkshopCardCalibrationAddDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<Self> {
        if params.no_of_calibration_records > u16::from(u8::MAX) {
            return Err(Error::RecordOutOfRange(format!(
                "Workshop calibration additional data record count: {}",
                params.no_of_calibration_records
            )));
        }
        let (calibration_pointer_newest_record, workshop_card_calibration_add_data_records) =
            read_card_records::<_, WorkshopCardCalibrationAddDataRecord>(
                reader,
                u32::from(params.no_of_calibration_records),
                27,
                "Workshop calibration additional data",
            )?;
        Ok(Self { calibration_pointer_newest_record, workshop_card_calibration_add_data_records })
    }
}
