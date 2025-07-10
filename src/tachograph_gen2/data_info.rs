use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};

use crate::{
    Error,
    error::Result,
    tacho::{DataTypeID, VUTransferResponseParameterID},
};

#[derive(Debug)]
pub struct DataConfig {
    pub data_type_id: DataTypeID,
    pub record_size: u16,
    pub no_of_records: u16,
}

#[derive(Debug)]
pub struct DataInfo {
    pub trep_id: VUTransferResponseParameterID,
    pub data_type_id: DataTypeID,
    pub record_size: u16,
    pub no_of_records: u16,
    pub data: Vec<u8>,
}

impl DataInfo {
    pub fn read<R: ReadBytes + BinSeek>(reader: &mut R, trep_id: VUTransferResponseParameterID) -> Result<DataInfo> {
        let data_type_id = DataTypeID::from(reader.read_u8()?);
        let data_size = reader.read_u16::<BigEndian>()?;
        let no_of_records = reader.read_u16::<BigEndian>()?;
        let full_data_size = data_size * no_of_records;
        let data = reader.read_into_vec(full_data_size as u32)?;

        Ok(DataInfo { trep_id, data_type_id, record_size: data_size, no_of_records, data })
    }

    pub fn parse<T: DataInfoReadable<T>>(&self) -> Result<T> {
        let config = DataConfig {
            data_type_id: self.data_type_id.clone(),
            record_size: self.record_size,
            no_of_records: self.no_of_records,
        };
        let mut reader = BinMemoryBuffer::from(self.data.clone());
        T::read(&mut reader, &config)
    }
}

pub trait DataInfoReadable<T> {
    fn read<R: ReadBytes + BinSeek>(_reader: &mut R, _config: &DataConfig) -> Result<T> {
        Err(Error::NotImplemented)
    }
}
