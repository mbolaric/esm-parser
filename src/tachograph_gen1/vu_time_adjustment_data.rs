use crate::{
    Readable,
    gen1::Address,
    tacho::{FullCardNumber, Name, TimeReal},
};

#[derive(Debug)]
pub struct VuTimeAdjustmentRecord {}

impl Readable<VuTimeAdjustmentRecord> for VuTimeAdjustmentRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuTimeAdjustmentRecord> {
        let OldTimeValue = TimeReal::read(reader)?;
        let NewTimeValue = TimeReal::read(reader)?;
        let WorkshopName = Name::read(reader)?;
        let WorkshopAddress = Address::read(reader)?;
        let WorkshopCardNumber = FullCardNumber::read(reader)?;

        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct VuTimeAdjustmentData {}

impl Readable<VuTimeAdjustmentData> for VuTimeAdjustmentData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuTimeAdjustmentData> {
        Ok(Self {})
    }
}
