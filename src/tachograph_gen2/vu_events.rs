use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecords, SignatureRecords, VuEventRecord, VuFaultRecord, VuOverSpeedingEventRecord,
    VuTimeAdjustmentRecord,
};
use crate::tacho::{VUTransferResponseParameterID, VuOverSpeedingControlData};

#[derive(Debug, Serialize)]
pub struct VUEvents {
    pub vu_fault_records: DataInfoGenericRecords<VuFaultRecord>,
    pub vu_event_records: DataInfoGenericRecords<VuEventRecord>,
    pub vu_over_speeding_control_data_records: DataInfoGenericRecords<VuOverSpeedingControlData>,
    pub vu_over_speeding_event_records: DataInfoGenericRecords<VuOverSpeedingEventRecord>,
    pub vu_time_adjustment_records: DataInfoGenericRecords<VuTimeAdjustmentRecord>,
    pub signature_records: Option<SignatureRecords>,
}

impl VUEvents {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUEvents> {
        debug!("VUEvents::from_data - Trep ID: {trep_id:?}");
        let vu_fault_records: DataInfoGenericRecords<VuFaultRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_event_records: DataInfoGenericRecords<VuEventRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_over_speeding_control_data_records: DataInfoGenericRecords<VuOverSpeedingControlData> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_over_speeding_event_records: DataInfoGenericRecords<VuOverSpeedingEventRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_time_adjustment_records: DataInfoGenericRecords<VuTimeAdjustmentRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_records: Option<SignatureRecords> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            vu_fault_records,
            vu_event_records,
            vu_over_speeding_control_data_records,
            vu_over_speeding_event_records,
            vu_time_adjustment_records,
            signature_records,
        })
    }
}
