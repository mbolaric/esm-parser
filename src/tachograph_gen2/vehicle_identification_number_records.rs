use binary_data::{BinSeek, ReadBytes};

use crate::{
    CodePage, Result, bytes_to_string, gen2::DataInfoReadable, tacho::DataTypeID, tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug)]
pub struct VehicleIdentificationNumberRecords {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: DataTypeID,
    pub records: Vec<String>,
}

impl DataInfoReadable<VehicleIdentificationNumberRecords> for VehicleIdentificationNumberRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VehicleIdentificationNumberRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<String> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = bytes_to_string(&reader.read_into_vec(record_size as u32)?, &CodePage::IsoIec8859_1);
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}
