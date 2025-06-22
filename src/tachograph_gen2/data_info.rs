use binary_data::{BigEndian, BinSeek, ReadBytes};

use crate::{
    error::Result,
    tacho::{DataTypeID, VUTransferResponseParameterID},
};

#[derive(Debug)]
pub struct DataInfo {
    pub trep_id: VUTransferResponseParameterID,
    pub data_type_id: DataTypeID,
    pub data_size: u16,
    pub no_of_records: u16,
    pub data: Vec<u8>,
}

impl DataInfo {
    pub fn read<R: ReadBytes + BinSeek>(
        reader: &mut R,
        trep_id: VUTransferResponseParameterID,
    ) -> Result<DataInfo> {
        let data_type_id = DataTypeID::from(reader.read_u8()?);
        let data_size = reader.read_u16::<BigEndian>()?;
        let no_of_records = reader.read_u16::<BigEndian>()?;
        let full_data_size = data_size * no_of_records;
        let data = reader.read_into_vec(full_data_size as u32)?;

        Ok(DataInfo {
            trep_id,
            data_type_id,
            data_size,
            no_of_records,
            data,
        })
    }
}
