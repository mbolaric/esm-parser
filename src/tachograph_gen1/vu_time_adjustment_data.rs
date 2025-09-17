use serde::Serialize;

use crate::{
    Readable,
    tacho::{Address, FullCardNumber, Name, TimeReal},
};

#[derive(Debug, Serialize)]
pub struct VuTimeAdjustmentRecord {
    pub old_time_value: TimeReal,
    pub mew_time_value: TimeReal,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumber,
}

impl Readable<VuTimeAdjustmentRecord> for VuTimeAdjustmentRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuTimeAdjustmentRecord> {
        let old_time_value = TimeReal::read(reader)?;
        let mew_time_value = TimeReal::read(reader)?;
        let workshop_name = Name::read(reader)?;
        let workshop_address = Address::read(reader)?;
        let workshop_card_number = FullCardNumber::read(reader)?;

        Ok(Self { old_time_value, mew_time_value, workshop_name, workshop_address, workshop_card_number })
    }
}

#[derive(Debug, Serialize)]
pub struct VuTimeAdjustmentData {
    pub no_of_vu_time_adj_records: u8,
    pub vu_time_adjustment_records: Vec<VuTimeAdjustmentRecord>,
}

impl Readable<VuTimeAdjustmentData> for VuTimeAdjustmentData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuTimeAdjustmentData> {
        let no_of_vu_time_adj_records = reader.read_u8()?;
        let mut vu_time_adjustment_records: Vec<VuTimeAdjustmentRecord> = Vec::new();
        for _ in 0..no_of_vu_time_adj_records {
            let record = VuTimeAdjustmentRecord::read(reader)?;
            vu_time_adjustment_records.push(record);
        }
        Ok(Self { no_of_vu_time_adj_records, vu_time_adjustment_records })
    }
}
