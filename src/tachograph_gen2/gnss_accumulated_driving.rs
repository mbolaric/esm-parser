use binary_data::{BigEndian, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Readable, ReadableWithParams, Result,
    gen2::GnssPlaceRecord,
    tacho::{OdometerShort, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct GnssAccumulatedDrivingRecord {
    pub time_stamp: TimeReal,
    pub gnss_place_record: GnssPlaceRecord,
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<GnssAccumulatedDrivingRecord> for GnssAccumulatedDrivingRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<GnssAccumulatedDrivingRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;
        Ok(Self { time_stamp, gnss_place_record, vehicle_odometer_value })
    }
}

#[derive(Debug)]
pub struct GnssAccumulatedDrivingParams {
    pub no_of_gnss_ad_records: u32,
}

impl GnssAccumulatedDrivingParams {
    pub fn new(no_of_gnss_ad_records: u32) -> Self {
        Self { no_of_gnss_ad_records }
    }
}

#[derive(Debug, Serialize)]
pub struct GnssAccumulatedDriving {
    pub gnss_ad_pointer_newest_record: u16,
    pub gnss_accumulated_driving_records: Vec<GnssAccumulatedDrivingRecord>,
}

impl ReadableWithParams<GnssAccumulatedDriving> for GnssAccumulatedDriving {
    type P = GnssAccumulatedDrivingParams;

    fn read<R: ReadBytes + BinSeek>(reader: &mut R, params: &Self::P) -> Result<GnssAccumulatedDriving> {
        let gnss_ad_pointer_newest_record = reader.read_u16::<BigEndian>()?;
        let mut records: Vec<GnssAccumulatedDrivingRecord> = Vec::new();
        for _ in 0..params.no_of_gnss_ad_records {
            let record = GnssAccumulatedDrivingRecord::read(reader)?;
            if record.time_stamp.has_data() {
                records.push(record);
            }
        }

        Ok(Self { gnss_ad_pointer_newest_record, gnss_accumulated_driving_records: records })
    }
}
