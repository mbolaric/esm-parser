use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::CardGeneration;
use crate::{Error, Readable, ReadableWithParams, Result, read_records};

#[derive(Debug)]
pub struct WorkshopCardCalibrationDataParams {
    pub no_of_calibration_records: u16,
    pub card_generation: CardGeneration,
}

impl WorkshopCardCalibrationDataParams {
    pub fn new(no_of_calibration_records: u16, card_generation: CardGeneration) -> Self {
        Self { no_of_calibration_records, card_generation }
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
    pub calibration_pointer_newest_record: u16,
    #[serde(rename = "calibrationRecords")]
    pub calibration_records: Vec<T>,
}

impl<T: Readable> ReadableWithParams for WorkshopCardCalibrationData<T> {
    type P = WorkshopCardCalibrationDataParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<WorkshopCardCalibrationData<T>> {
        let (pointer_size, record_size) = match params.card_generation {
            CardGeneration::Gen1 => (1, 105),
            CardGeneration::Gen2 => (2, 178),
            CardGeneration::Combined => {
                return Err(Error::InvalidDataParse("Workshop calibration requires a single application generation".to_owned()));
            }
        };
        let count = usize::from(params.no_of_calibration_records);
        if count == 0 || count > usize::from(u8::MAX) {
            return Err(Error::RecordOutOfRange(format!("Workshop calibration record count: {count}")));
        }
        let expected_size = 2 + pointer_size + count * record_size;
        let remaining = reader.len()?.saturating_sub(reader.pos()?);
        if remaining != expected_size {
            return Err(Error::InvalidDataParse(format!(
                "Workshop calibration EF size: expected {expected_size} bytes, got {remaining}"
            )));
        }
        let calibration_total_number = reader.read_u16::<BigEndian>()?;
        let calibration_pointer_newest_record = if params.card_generation == CardGeneration::Gen1 {
            u16::from(reader.read_u8()?)
        } else {
            reader.read_u16::<BigEndian>()?
        };
        if usize::from(calibration_pointer_newest_record) >= count {
            return Err(Error::RecordOutOfRange(format!(
                "Workshop calibration newest-record pointer: {calibration_pointer_newest_record}, capacity: {count}"
            )));
        }
        let calibration_records = read_records(reader, count, |reader| {
            // Gen2 physical slots reserve room for five seals, even when fewer are recorded (TCS_162).
            let mut slot = BinMemoryBuffer::from(reader.read_into_vec(record_size as u32)?);
            let record = T::read(&mut slot)?;
            if params.card_generation == CardGeneration::Gen1 && slot.pos()? != record_size {
                return Err(Error::InvalidDataParse("Gen1 workshop calibration record did not consume its slot".to_owned()));
            }
            Ok(record)
        })?;
        Ok(Self { calibration_total_number, calibration_pointer_newest_record, calibration_records })
    }
}
