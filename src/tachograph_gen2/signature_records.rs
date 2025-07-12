use binary_data::{BinSeek, ReadBytes};

use crate::{
    Result, gen2::DataInfoReadable, helpers::vec_u8_to_string, tacho::DataTypeID, tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug)]
pub struct SignatureRecords {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: DataTypeID,
    pub records: Vec<Vec<u8>>,
}

impl DataInfoReadable<SignatureRecords> for SignatureRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<SignatureRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<Vec<u8>> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = reader.read_into_vec(record_size as u32)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}
