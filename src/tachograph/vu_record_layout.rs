use crate::tacho::{RecordType, VUTransferResponseParameterID};

/// The tag and physical size of a record in a Gen2 VU download array.
pub trait VuRecordLayout {
    const RECORD_TYPE: RecordType;
    const RECORD_SIZE: u16;

    fn record_type(_trep_id: VUTransferResponseParameterID) -> RecordType {
        Self::RECORD_TYPE
    }

    fn record_size(_trep_id: VUTransferResponseParameterID) -> u16 {
        Self::RECORD_SIZE
    }
}
