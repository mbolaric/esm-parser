use binary_data::{BinSeek, ReadBytes};

use crate::{
    Readable, Result,
    gen2::{DataInfoReadable, VuIdentification},
    tacho::{DataTypeID, VUTransferResponseParameterID},
    tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug)]
pub struct VuIdentificationRecords {
    pub is_gen2_v2: bool,
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: DataTypeID,
    pub records: Vec<VuIdentification>,
}

impl DataInfoReadable<VuIdentificationRecords> for VuIdentificationRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuIdentificationRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<VuIdentification> = Vec::with_capacity(no_of_records as usize);
        let is_gen2_v2: bool = config.trep_id == VUTransferResponseParameterID::Gen2v2Activities;
        for _ in 0..no_of_records {
            let record = VuIdentification::read(reader)?;
            records.push(record);
            if is_gen2_v2 {
                let _ = reader.read_bytes::<12>()?;
            }
        }
        Ok(Self { is_gen2_v2, no_of_records, record_size, data_type_id, records })
    }
}
