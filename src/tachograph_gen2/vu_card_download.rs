use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{CardData, DataInfo, SignatureRecordArray};
use crate::tacho::{TachographHeader, VUTransferResponseParameterID};

#[derive(Debug, Serialize)]
pub struct VUCardDownload {
    pub card: CardData,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUCardDownload {
    pub fn from_data<R: ReadBytes + BinSeek>(
        header: TachographHeader,
        trep_id: VUTransferResponseParameterID,
        reader: &mut R,
    ) -> Result<VUCardDownload> {
        debug!("VUCardDownload::from_data - Trep ID: {trep_id:?}");
        let data_info = DataInfo::read(reader, trep_id.clone())?;
        let card: CardData = CardData::from_data(header, &mut data_info.data_into_mem_reader()?)?;
        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self { card, signature_record_array })
    }
}
