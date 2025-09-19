use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    Error, Readable, ReadableWithParams,
    error::Result,
    tacho::{RecordType, VUTransferResponseParameterID},
};

#[derive(Debug)]
pub struct DataConfig {
    pub trep_id: VUTransferResponseParameterID,
    pub data_type_id: RecordType,
    pub record_size: u16,
    pub no_of_records: u16,
}

#[derive(Debug, Serialize)]
pub struct DataInfo {
    pub trep_id: VUTransferResponseParameterID,
    pub data_type_id: RecordType,
    pub record_size: u16,
    pub no_of_records: u16,
    pub data: Vec<u8>,
}

impl DataInfo {
    fn create_data_config(&self) -> DataConfig {
        DataConfig {
            trep_id: self.trep_id.clone(),
            data_type_id: self.data_type_id.clone(),
            record_size: self.record_size,
            no_of_records: self.no_of_records,
        }
    }

    pub fn data_into_mem_reader(&self) -> Result<BinMemoryBuffer> {
        Ok(BinMemoryBuffer::from(self.data.clone()))
    }

    pub fn read<R: ReadBytes + BinSeek>(reader: &mut R, trep_id: VUTransferResponseParameterID) -> Result<DataInfo> {
        let data_type_id = RecordType::from(reader.read_u8()?);
        let data_size = reader.read_u16::<BigEndian>()?;
        let no_of_records = reader.read_u16::<BigEndian>()?;
        let full_data_size: u32 = data_size as u32 * no_of_records as u32;
        let data = reader.read_into_vec(full_data_size)?;

        Ok(DataInfo { trep_id, data_type_id, record_size: data_size, no_of_records, data })
    }

    pub fn parse<T: DataInfoReadable<T>>(&self) -> Result<T> {
        let config = self.create_data_config();
        let mut reader = BinMemoryBuffer::from(self.data.clone());
        T::read(&mut reader, &config)
    }

    pub fn parse_with_params<T: DataInfoReadableWithParams<T>>(&self) -> Result<T> {
        let config = self.create_data_config();
        let mut reader = BinMemoryBuffer::from(self.data.clone());
        T::read(&mut reader, &config)
    }
}

pub trait DataInfoReadable<T> {
    fn read<R: ReadBytes + BinSeek>(_reader: &mut R, _config: &DataConfig) -> Result<T> {
        Err(Error::NotImplemented)
    }
}

pub trait DataInfoReadableWithParams<T> {
    fn read<R: ReadBytes + BinSeek>(_reader: &mut R, _config: &DataConfig) -> Result<T> {
        Err(Error::NotImplemented)
    }
}

#[derive(Debug, Serialize)]
pub struct DataInfoGenericRecords<T> {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: RecordType,
    pub records: Vec<T>,
}

impl<T: Readable<T>> DataInfoReadable<DataInfoGenericRecords<T>> for DataInfoGenericRecords<T> {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecords<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<T> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = T::read(reader)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}

impl<T: ReadableWithParams<T, P = VUTransferResponseParameterID>> DataInfoReadableWithParams<DataInfoGenericRecords<T>>
    for DataInfoGenericRecords<T>
{
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecords<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<T> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = T::read(reader, &config.trep_id)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}
