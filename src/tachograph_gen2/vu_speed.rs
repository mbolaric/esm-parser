use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{DataInfo, DataInfoGenericRecords, SignatureRecords};
use crate::tacho::{VUTransferResponseParameterID, VuDetailedSpeedBlock};

#[derive(Debug)]
pub struct VUSpeed {
    pub vu_detailed_speed_block_records: DataInfoGenericRecords<VuDetailedSpeedBlock>,
    pub signature_records: Option<SignatureRecords>,
}

impl VUSpeed {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUSpeed> {
        debug!("VUSpeed::from_data - Trep ID: {:?}", trep_id);
        let vu_detailed_speed_block_records: DataInfoGenericRecords<VuDetailedSpeedBlock> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_records: Option<SignatureRecords> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self { vu_detailed_speed_block_records, signature_records })
    }
}
