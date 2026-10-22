use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};
use serde::Serialize;

use crate::tacho::{RecordType, VUTransferResponseParameterID};
use crate::{Error, Readable, ReadableWithParams, Result, read_records};

#[derive(Debug)]
pub struct DataConfig {
    pub trep_id: VUTransferResponseParameterID,
    pub record_type: RecordType,
    pub record_size: u16,
    pub no_of_records: u16,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2DataInfo"))]
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
            trep_id: self.trep_id,
            record_type: self.record_type,
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

    pub fn parse<T: DataInfoReadable>(&self) -> Result<T> {
        let config = self.create_data_config();
        let mut reader = BinMemoryBuffer::from(self.data.clone());
        T::read(&mut reader, &config)
    }

    pub(crate) fn parse_record_array<T: Readable>(
        &self,
        record_type: RecordType,
        record_size: u16,
    ) -> Result<DataInfoGenericRecordArray<T>> {
        if self.record_type != record_type || self.record_size != record_size {
            return Err(Error::InvalidDataParse(format!(
                "Record array layout: expected {record_type:?} with {record_size} bytes, got {:?} with {} bytes",
                self.record_type, self.record_size
            )));
        }
        let expected_size = usize::from(record_size) * usize::from(self.no_of_records);
        if self.data.len() != expected_size {
            return Err(Error::InvalidDataParse(format!(
                "Record array payload: expected {expected_size} bytes, got {}",
                self.data.len()
            )));
        }
        self.parse()
    }

    pub fn parse_with_params<T: DataInfoReadableWithParams>(&self) -> Result<T> {
        let config = self.create_data_config();
        let mut reader = BinMemoryBuffer::from(self.data.clone());
        T::read(&mut reader, &config)
    }
}

pub trait DataInfoReadable: Sized {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<Self>;
}

pub trait DataInfoReadableWithParams: Sized {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<Self>;
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(rename = "Gen2DataInfoGenericRecordArray"))]
pub struct DataInfoGenericRecordArray<T> {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<T>,
}

impl<T: Readable> DataInfoReadable for DataInfoGenericRecordArray<T> {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecordArray<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;

        let records = read_records(reader, no_of_records as usize, |reader| T::read(reader))?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}

impl<T: ReadableWithParams<P = VUTransferResponseParameterID>> DataInfoReadableWithParams for DataInfoGenericRecordArray<T> {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<DataInfoGenericRecordArray<T>> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type;

        let records = read_records(reader, no_of_records as usize, |reader| T::read(reader, &config.trep_id))?;
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen2::VuBorderCrossingRecord;

    #[test]
    fn fixed_record_arrays_reject_payloads_that_disagree_with_the_header() {
        let mut info = DataInfo {
            trep_id: VUTransferResponseParameterID::Gen2v2Activities,
            record_type: RecordType::VuBorderCrossingRecord,
            record_size: 55,
            no_of_records: 1,
            data: vec![0; 55],
        };
        assert!(info.parse_record_array::<VuBorderCrossingRecord>(RecordType::VuBorderCrossingRecord, 55).is_ok());
        for length in [0, 54, 56] {
            info.data.resize(length, 0);
            assert!(matches!(
                info.parse_record_array::<VuBorderCrossingRecord>(RecordType::VuBorderCrossingRecord, 55),
                Err(Error::InvalidDataParse(message)) if message.contains("payload")
            ));
        }
    }
}
