use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::{GnssPlaceRecord, OperationType};
use crate::tacho::{NationNumeric, OdometerShort, RegionNumeric, TimeReal};
use crate::{Readable, ReadableWithParams, Result};

/// Information, stored in a driver or workshop card, related to load/unload operations
/// (Annex IC requirements 306g and 356g)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadUnloadRecord"))]
pub struct LoadUnloadRecord {
    #[serde(rename = "operationType")]
    pub operation_type: OperationType,
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    pub country: NationNumeric,
    pub region: RegionNumeric,
    #[serde(rename = "gnssPlaceRecord")]
    pub gnss_place_record: GnssPlaceRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<LoadUnloadRecord> for LoadUnloadRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<LoadUnloadRecord> {
        let operation_type: OperationType = reader.read_u8()?.into();
        let time_stamp = TimeReal::read(reader)?;
        let country: NationNumeric = reader.read_u8()?.into();
        let region: RegionNumeric = reader.read_u8()?.into();
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;
        Ok(Self { operation_type, time_stamp, country, region, gnss_place_record, vehicle_odometer_value })
    }
}

#[derive(Debug)]
pub struct LoadUnloadOperationsParams {
    pub no_of_load_unload_records: u16,
}

impl LoadUnloadOperationsParams {
    pub fn new(no_of_load_unload_records: u16) -> Self {
        Self { no_of_load_unload_records }
    }
}

/// Information, stored in a driver or workshop card, related to load/unload operations
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadUnloadOperations"))]
pub struct LoadUnloadOperations {
    #[serde(rename = "loadUnloadPointerNewestRecord")]
    pub load_unload_pointer_newest_record: u16,
    #[serde(rename = "cardLoadUnloadRecords")]
    pub card_load_unload_records: Vec<LoadUnloadRecord>,
}

impl ReadableWithParams<LoadUnloadOperations> for LoadUnloadOperations {
    type P = LoadUnloadOperationsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<LoadUnloadOperations> {
        let load_unload_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut records: Vec<LoadUnloadRecord> = Vec::new();
        for _ in 0..params.no_of_load_unload_records {
            let record = LoadUnloadRecord::read(reader)?;
            if record.time_stamp.has_data() {
                records.push(record);
            }
        }

        Ok(Self { load_unload_pointer_newest_record, card_load_unload_records: records })
    }
}
