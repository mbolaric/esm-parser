use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::{DataInfoReadable, FullCardNumberAndGeneration, GnssPlaceRecord, PlaceRecord},
    tacho::{DataTypeID, OdometerShort, TimeReal, VUTransferResponseParameterID},
    tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug)]
pub struct VuGnssadRecord {
    pub time_stamp: TimeReal,
    pub driver_slot_card_number_and_gen: FullCardNumberAndGeneration,
    pub co_driver_slot_card_number_and_gen: FullCardNumberAndGeneration,
    pub gnss_place_record: GnssPlaceRecord,
    pub vehicle_odometer_value: OdometerShort,
}

impl Readable<VuGnssadRecord> for VuGnssadRecord {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R) -> Result<VuGnssadRecord> {
        let time_stamp = TimeReal::read(reader)?;
        let driver_slot_card_number_and_gen = FullCardNumberAndGeneration::read(reader)?;
        let co_driver_slot_card_number_and_gen = FullCardNumberAndGeneration::read(reader)?;
        let gnss_place_record = GnssPlaceRecord::read(reader)?;
        let vehicle_odometer_value = OdometerShort::read(reader)?;

        Ok(Self {
            time_stamp,
            driver_slot_card_number_and_gen,
            co_driver_slot_card_number_and_gen,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug)]
pub struct VuGnssadRecords {
    pub is_gen2_v2: bool,
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: DataTypeID,
    pub records: Vec<VuGnssadRecord>,
}

impl DataInfoReadable<VuGnssadRecords> for VuGnssadRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuGnssadRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<VuGnssadRecord> = Vec::with_capacity(no_of_records as usize);
        let is_gen2_v2: bool = config.trep_id == VUTransferResponseParameterID::Gen2v2Activities;
        for _ in 0..no_of_records {
            let record = VuGnssadRecord::read(reader)?;
            records.push(record);
            if is_gen2_v2 {
                let _ = reader.read_u8()?;
            }
        }
        Ok(Self { is_gen2_v2, no_of_records, record_size, data_type_id, records })
    }
}
