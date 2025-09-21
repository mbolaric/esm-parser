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
    pub record_type: RecordType,
    pub record_size: u16,
    pub no_of_records: u16,
}

#[derive(Debug, Serialize)]
pub struct DataInfo {
    #[serde(rename = "trepId")]
    pub trep_id: VUTransferResponseParameterID,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    pub data: Vec<u8>,
}

impl DataInfo {
    fn create_data_config(&self) -> DataConfig {
        DataConfig {
            trep_id: self.trep_id.clone(),
            record_type: self.record_type.clone(),
            record_size: self.record_size,
            no_of_records: self.no_of_records,
        }
    }

    pub fn data_into_mem_reader(&self) -> Result<BinMemoryBuffer> {
        Ok(BinMemoryBuffer::from(self.data.clone()))
    }

    pub fn read<R: ReadBytes + BinSeek>(reader: &mut R, trep_id: VUTransferResponseParameterID) -> Result<DataInfo> {
        let record_type = RecordType::from(reader.read_u8()?);
        let data_size = reader.read_u16::<BigEndian>()?;
        let no_of_records = reader.read_u16::<BigEndian>()?;
        let full_data_size: u32 = data_size as u32 * no_of_records as u32;
        let data = reader.read_into_vec(full_data_size)?;

        Ok(DataInfo { trep_id, record_type, record_size: data_size, no_of_records, data })
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
pub struct DataInfoGenericRecordArray<T> {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<T>,
}

impl<T: Readable<T>> DataInfoReadable<DataInfoGenericRecordArray<T>> for DataInfoGenericRecordArray<T> {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecordArray<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<T> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = T::read(reader)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}

impl<T: ReadableWithParams<T, P = VUTransferResponseParameterID>> DataInfoReadableWithParams<DataInfoGenericRecordArray<T>>
    for DataInfoGenericRecordArray<T>
{
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecordArray<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<T> = Vec::with_capacity(no_of_records as usize);
        for _ in 0..no_of_records {
            let record = T::read(reader, &config.trep_id)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
