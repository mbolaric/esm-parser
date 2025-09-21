use binary_data::{BinSeek, ReadBytes};
use log::debug;
use serde::Serialize;

use crate::Result;
use crate::gen2::{
    DataInfo, DataInfoGenericRecordArray, SignatureRecordArray, VuEventRecord, VuFaultRecord, VuOverSpeedingEventRecord,
    VuTimeAdjustmentRecord,
};
use crate::tacho::{VUTransferResponseParameterID, VuOverSpeedingControlData};

/// Data structure generation 2, version 2 (TREP 33 Hex)
#[derive(Debug, Serialize)]
pub struct VUEvents {
    #[serde(rename = "vuFaultRecordArray")]
    pub vu_fault_record_array: DataInfoGenericRecordArray<VuFaultRecord>,
    #[serde(rename = "vuEventRecordArray")]
    pub vu_event_record_array: DataInfoGenericRecordArray<VuEventRecord>,
    #[serde(rename = "vuOverSpeedingControlDataRecordArray")]
    pub vu_over_speeding_control_data_record_array: DataInfoGenericRecordArray<VuOverSpeedingControlData>,
    #[serde(rename = "VuOverSpeedingEventRecordArray")]
    pub vu_over_speeding_event_record_array: DataInfoGenericRecordArray<VuOverSpeedingEventRecord>,
    #[serde(rename = "vuTimeAdjustmentRecordArray")]
    pub vu_time_adjustment_record_array: DataInfoGenericRecordArray<VuTimeAdjustmentRecord>,
    #[serde(rename = "signatureRecordArray")]
    pub signature_record_array: Option<SignatureRecordArray>,
}

impl VUEvents {
    pub fn from_data<R: ReadBytes + BinSeek>(trep_id: VUTransferResponseParameterID, reader: &mut R) -> Result<VUEvents> {
        debug!("VUEvents::from_data - Trep ID: {trep_id:?}");
        let vu_fault_records: DataInfoGenericRecordArray<VuFaultRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_event_records: DataInfoGenericRecordArray<VuEventRecord> = DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_over_speeding_control_data_record_array: DataInfoGenericRecordArray<VuOverSpeedingControlData> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_over_speeding_event_record_array: DataInfoGenericRecordArray<VuOverSpeedingEventRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let vu_time_adjustment_record_array: DataInfoGenericRecordArray<VuTimeAdjustmentRecord> =
            DataInfo::read(reader, trep_id.clone())?.parse()?;
        let signature_record_array: Option<SignatureRecordArray> = Some(DataInfo::read(reader, trep_id.clone())?.parse()?);

        Ok(Self {
            vu_fault_record_array: vu_fault_records,
            vu_event_record_array: vu_event_records,
            vu_over_speeding_control_data_record_array,
            vu_over_speeding_event_record_array,
            vu_time_adjustment_record_array,
            signature_record_array,
        })
    }
}
