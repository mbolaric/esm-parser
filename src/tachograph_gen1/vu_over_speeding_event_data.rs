use crate::{
    Readable,
    tacho::{EventFaultRecordPurpose, EventFaultType, FullCardNumber, TimeReal},
};

#[derive(Debug)]
pub struct VuOverSpeedingEventRecord {}

impl Readable<VuOverSpeedingEventRecord> for VuOverSpeedingEventRecord {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingEventRecord> {
        let EventType: EventFaultType = reader.read_u8()?.into();
        let EventRecordPurpose: EventFaultRecordPurpose = reader.read_u8()?.into();
        let EventBeginTime = TimeReal::read(reader)?;
        let EventEndTime = TimeReal::read(reader)?;
        let MaxSpeedValue = reader.read_u8()?;
        let AverageSpeedValue = reader.read_u8()?;
        let CardNumberDriverSlotBegin = FullCardNumber::read(reader)?;
        let SimilarEventsNumber = reader.read_u8()?;

        Ok(Self {})
    }
}

#[derive(Debug)]
pub struct VuOverSpeedingEventData {}

impl Readable<VuOverSpeedingEventData> for VuOverSpeedingEventData {
    fn read<R: binary_data::ReadBytes + binary_data::BinSeek>(reader: &mut R) -> crate::Result<VuOverSpeedingEventData> {
        Ok(Self {})
    }
}
