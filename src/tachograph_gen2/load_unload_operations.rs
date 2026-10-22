use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::gen2::{GnssPlaceAuthRecord, OperationType};
use crate::tacho::{OdometerShort, TimeReal};
use crate::tachograph_gen2::card_records::read_card_records;
use crate::{Readable, ReadableWithParams, Result};

/// Information, stored in a driver or workshop card, related to load/unload operations
/// (Annex IC requirements 306g and 356g)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadUnloadRecord"))]
pub struct LoadUnloadRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "operationType")]
    pub operation_type: OperationType,
    #[serde(rename = "gnssPlaceAuthRecord")]
    pub gnss_place_auth_record: GnssPlaceAuthRecord,
    #[serde(rename = "vehicleOdometerValue")]
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable for LoadUnloadRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<LoadUnloadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let operation_type: OperationType = reader.read_u8()?.into();
        let gnss_place_auth_record = GnssPlaceAuthRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;
        Ok(Self { time_stamp, operation_type, gnss_place_auth_record, vehicle_odometer_value })
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
/// (Annex IC requirements 306h and 356h; Appendix 1, section 2.24c).
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2LoadUnloadOperations"))]
pub struct LoadUnloadOperations {
    #[serde(rename = "loadUnloadPointerNewestRecord")]
    pub load_unload_pointer_newest_record: u16,
    #[serde(rename = "cardLoadUnloadRecords")]
    pub card_load_unload_records: Vec<LoadUnloadRecord>,
}

impl ReadableWithParams for LoadUnloadOperations {
    type P = LoadUnloadOperationsParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<LoadUnloadOperations> {
        let (load_unload_pointer_newest_record, records) = read_card_records::<_, LoadUnloadRecord>(
            reader,
            u32::from(params.no_of_load_unload_records),
            20,
            "Load/unload operations",
        )?;
        let records = records.into_iter().filter(|record| record.time_stamp.has_data() && record.time_stamp.is_set()).collect();

        Ok(Self { load_unload_pointer_newest_record, card_load_unload_records: records })
    }
}

#[cfg(test)]
mod tests {
    use binary_data::BinMemoryBuffer;
    use serde_json::json;

    use super::*;

    const LOAD: [u8; 20] = [0, 0, 0, 100, 1, 0, 0, 0, 90, 1, 0, 0xB9, 0x94, 0, 0x0D, 6, 0, 1, 0xE2, 0x40];
    const UNLOAD: [u8; 20] = [0, 0, 0, 200, 2, 0, 0, 0, 190, 7, 0, 0x0D, 6, 0, 0xB9, 0x94, 1, 9, 0xFB, 0xF1];
    const BOTH: [u8; 20] = [0, 0, 1, 44, 3, 0, 0, 1, 34, 3, 0, 0, 0, 0, 0, 0, 0xFE, 0, 0, 42];

    #[test]
    fn reads_adjacent_twenty_byte_records_for_all_three_operations() {
        let mut reader = BinMemoryBuffer::from([LOAD.as_slice(), UNLOAD.as_slice(), BOTH.as_slice()].concat());
        for (index, operation, timestamp, accuracy, status, odometer) in
            [(0, "Load", 100, 1, 0, 123456), (1, "Unload", 200, 7, 1, 654321), (2, "SimultaneousLoadAndUnload", 300, 3, 0xFE, 42)]
        {
            let record = LoadUnloadRecord::read(&mut reader).unwrap();
            assert_eq!(reader.pos().unwrap(), (index + 1) * 20);
            let value = serde_json::to_value(record).unwrap();
            assert_eq!(value.as_object().unwrap().len(), 4);
            assert_eq!(value["timeStamp"], json!(TimeReal::new(timestamp)));
            assert_eq!(value["operationType"], operation);
            assert_eq!(value["vehicleOdometerValue"], odometer);
            let gnss = &value["gnssPlaceAuthRecord"];
            assert_eq!(gnss["timeStamp"], json!(TimeReal::new(timestamp - 10)));
            assert_eq!(gnss["gnssAccuracy"], accuracy);
            assert_eq!(gnss["authenticationStatus"], status);
            assert!(value.get("country").is_none());
            assert!(value.get("region").is_none());
            assert!(value.get("gnssPlaceRecord").is_none());
        }
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
        for length in 0..20 {
            assert!(LoadUnloadRecord::read(&mut BinMemoryBuffer::from(LOAD[..length].to_vec())).is_err());
        }
    }

    #[test]
    fn container_filters_by_operation_time_and_preserves_the_physical_pointer() {
        let mut no_operation_time = LOAD;
        no_operation_time[..4].fill(0);
        let mut no_gnss_time = UNLOAD;
        no_gnss_time[5..9].fill(0);
        let bytes = [
            4_u16.to_be_bytes().as_slice(),
            &[0; 20],
            LOAD.as_slice(),
            &[0xFF; 20],
            no_operation_time.as_slice(),
            no_gnss_time.as_slice(),
        ]
        .concat();
        let mut reader = BinMemoryBuffer::from(bytes);
        let operations = LoadUnloadOperations::read(&mut reader, &LoadUnloadOperationsParams::new(5)).unwrap();
        assert_eq!(operations.load_unload_pointer_newest_record, 4);
        assert_eq!(operations.card_load_unload_records.len(), 2);
        let records = &operations.card_load_unload_records;
        assert_eq!(records[0].time_stamp.data, 100);
        assert_eq!(records[1].time_stamp.data, 200);
        assert_eq!(reader.pos().unwrap(), reader.len().unwrap());
    }

    #[test]
    fn container_rejects_invalid_lengths_capacities_and_pointers() {
        let valid = [1_u16.to_be_bytes().as_slice(), LOAD.as_slice(), UNLOAD.as_slice()].concat();
        for length in [0, 1, 2, 21, 41, 43, 44] {
            let mut bytes = valid.clone();
            bytes.resize(length, 0);
            assert!(LoadUnloadOperations::read(&mut BinMemoryBuffer::from(bytes), &LoadUnloadOperationsParams::new(2)).is_err());
        }
        for pointer in [2_u16, u16::MAX] {
            let mut bytes = valid.clone();
            bytes[..2].copy_from_slice(&pointer.to_be_bytes());
            assert!(LoadUnloadOperations::read(&mut BinMemoryBuffer::from(bytes), &LoadUnloadOperationsParams::new(2)).is_err());
        }
        for count in [0, u16::MAX] {
            assert!(
                LoadUnloadOperations::read(&mut BinMemoryBuffer::from(valid.clone()), &LoadUnloadOperationsParams::new(count))
                    .is_err()
            );
        }
    }
}
