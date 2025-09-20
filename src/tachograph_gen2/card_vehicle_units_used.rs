use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{Readable, ReadableWithParams, Result, bytes_to_ia5_fix_string, tacho::TimeReal};

/// Information, stored in a driver or workshop card, related to a vehicle
/// unit that was used (Annex 1C requirement 303 and 351).
#[derive(Debug, Serialize)]
pub struct CardVehicleUnitRecord {
    #[serde(rename = "timeStamp")]
    pub time_stamp: TimeReal,
    #[serde(rename = "manufacturerCode")]
    pub manufacturer_code: u8,
    #[serde(rename = "deviceID")]
    pub device_id: u8,
    #[serde(rename = "vuSoftwareVersion")]
    pub vu_software_version: String,
}

impl Readable<CardVehicleUnitRecord> for CardVehicleUnitRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<CardVehicleUnitRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let manufacturer_code = reader.read_u8()?;
        let device_id = reader.read_u8()?;
        let vu_software_version = bytes_to_ia5_fix_string(&reader.read_into_vec(4)?)?;
        Ok(Self { time_stamp, manufacturer_code, device_id, vu_software_version })
    }
}

#[derive(Debug)]
pub struct CardVehicleUnitsUsedParams {
    pub no_of_card_vehicle_unit_records: u32,
}

impl CardVehicleUnitsUsedParams {
    pub fn new(no_of_card_vehicle_unit_records: u32) -> Self {
        Self { no_of_card_vehicle_unit_records }
    }
}

/// Information, stored in a driver or workshop card, related to the vehicle
/// units used by the card holder (Annex IC requirements 304 and 352).
#[derive(Debug, Serialize)]
pub struct CardVehicleUnitsUsed {
    #[serde(rename = "vehicleUnitPointerNewestRecord")]
    pub vehicle_unit_pointer_newest_record: u16,
    #[serde(rename = "cardVehicleUnitRecords")]
    pub card_vehicle_unit_records: Vec<CardVehicleUnitRecord>,
}

impl ReadableWithParams<CardVehicleUnitsUsed> for CardVehicleUnitsUsed {
    type P = CardVehicleUnitsUsedParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<CardVehicleUnitsUsed> {
        let vehicle_unit_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut records: Vec<CardVehicleUnitRecord> = Vec::new();
        for _ in 0..params.no_of_card_vehicle_unit_records {
            let record = CardVehicleUnitRecord::read(reader)?;
            if record.time_stamp.has_data() {
                records.push(record);
            }
        }

        Ok(Self { vehicle_unit_pointer_newest_record, card_vehicle_unit_records: records })
    }
}
