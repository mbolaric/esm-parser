use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    gen2::{Certificate, CertificateParams, DataInfoReadable},
    tacho::RecordType,
    tachograph_gen2::data_info::DataConfig,
};

#[derive(Debug, Serialize)]
pub struct VuCertificateRecords {
    pub no_of_records: u16,
    pub record_size: u16,
    pub data_type_id: RecordType,
    pub records: Vec<Certificate>,
}

impl DataInfoReadable<VuCertificateRecords> for VuCertificateRecords {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuCertificateRecords> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let data_type_id = config.data_type_id.clone();

        let mut records: Vec<Certificate> = Vec::with_capacity(no_of_records as usize);
        let params = CertificateParams::new(Some(record_size));
        for _ in 0..no_of_records {
            let record = Certificate::read(reader, &params)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, data_type_id, records })
    }
}
