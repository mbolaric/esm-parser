use binary_data::{BinSeek, ReadBytes};
use serde::Serialize;

use crate::{
    ReadableWithParams, Result,
    gen2::{Certificate, CertificateParams, DataInfoReadable},
    tacho::RecordType,
    tachograph_gen2::data_info::DataConfig,
};

/// The VU certificate plus metadata as used in the download protocol.
#[derive(Debug, Serialize)]
pub struct VuCertificateRecordArray {
    #[serde(rename = "noOfRecords")]
    pub no_of_records: u16,
    #[serde(rename = "recordSize")]
    pub record_size: u16,
    #[serde(rename = "recordType")]
    pub record_type: RecordType,
    pub records: Vec<Certificate>,
}

impl DataInfoReadable<VuCertificateRecordArray> for VuCertificateRecordArray {
    fn read<R: ReadBytes + BinSeek>(reader: &mut R, config: &DataConfig) -> Result<VuCertificateRecordArray> {
        let no_of_records = config.no_of_records;
        let record_size = config.record_size;
        let record_type = config.record_type.clone();

        let mut records: Vec<Certificate> = Vec::with_capacity(no_of_records as usize);
        let params = CertificateParams::new(Some(record_size));
        for _ in 0..no_of_records {
            let record = Certificate::read(reader, &params)?;
            records.push(record);
        }
        Ok(Self { no_of_records, record_size, record_type, records })
    }
}
