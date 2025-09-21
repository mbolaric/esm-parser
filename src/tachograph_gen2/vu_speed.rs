use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{DataInfo, DataInfoGenericRecordArray, SignatureRecordArray};
use crate::tacho::{VUTransferResponseParameterID, VuDetailedSpeedBlock};

#[derive(Debug, Serialize)]
pub struct VUSpeed {
    pub vu_detailed_speed_block_records: DataInfoGenericRecordArray<VuDetailedSpeedBlock>,
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUSpeed {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUSpeed> {
        debug!("VUSpeed::from_data - Trep ID: {trep_id:?}");
        let vu_detailed_speed_block_records: DataInfoGenericRecordArray<VuDetailedSpeedBlock> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self { vu_detailed_speed_block_records, signature_record_array })
    }
}
