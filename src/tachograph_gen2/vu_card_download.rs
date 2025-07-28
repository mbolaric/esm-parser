use binary_data::{BinSeek, ReadBytes};
use log::debug;

use crate::Result;
use crate::gen2::{CardData, DataInfo, SignatureRecords};
use crate::tacho::{TachographHeader, VUTransferResponseParameterID};

#[derive(Debug)]
pub struct VUCardDownload {
    pub card: CardData,
    pub signature_records: Option<SignatureRecords>,
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
        let signature_records: Option<SignatureRecords> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self { card, signature_records })
    }
}
